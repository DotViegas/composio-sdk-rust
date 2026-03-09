# Arquitetura do Sistema de Arquivos

## 🏗️ Visão Geral da Arquitetura

```
┌─────────────────────────────────────────────────────────────────┐
│                         USUÁRIO / AGENTE IA                      │
│  "Envie este relatório.pdf por email para cliente@empresa.com"  │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      COMPOSIO SDK (Rust)                         │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ FileHelper                                               │   │
│  │ - Processa schemas de ferramentas                        │   │
│  │ - Identifica campos file_uploadable/file_downloadable   │   │
│  │ - Transforma schemas para formato simples               │   │
│  └─────────────────────────────────────────────────────────┘   │
│                             │                                     │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ FileUploadable                                           │   │
│  │ - from_path(): Upload de arquivo local                  │   │
│  │ - from_url(): Upload de URL pública                     │   │
│  │ - Calcula MD5 hash                                       │   │
│  │ - Detecta MIME type                                      │   │
│  │ - Valida tamanho e permissões                           │   │
│  └─────────────────────────────────────────────────────────┘   │
│                             │                                     │
│                             ▼                                     │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ HTTP Client (reqwest)                                    │   │
│  │ - POST /api/v3/files/upload/request                     │   │
│  │ - Recebe presigned URL do S3                            │   │
│  └─────────────────────────────────────────────────────────┘   │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    COMPOSIO BACKEND API                          │
│                                                                   │
│  1. Valida API key                                               │
│  2. Verifica MD5 hash (deduplicação)                            │
│  3. Gera presigned URL do S3                                    │
│  4. Retorna: { id, key, type, new_presigned_url }              │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      AMAZON S3 STORAGE                           │
│                                                                   │
│  - Recebe arquivo via PUT request                               │
│  - Armazena com chave única                                     │
│  - Gera URLs temporárias para download                          │
│  - Gerencia expiração de arquivos                               │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    COMPOSIO TOOL ROUTER                          │
│                                                                   │
│  - Recebe parâmetros com s3key                                  │
│  - Baixa arquivo de S3 se necessário                            │
│  - Executa ferramenta (Gmail, Slack, etc.)                      │
│  - Retorna resultado (pode incluir arquivos)                    │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      API EXTERNA (Gmail)                         │
│                                                                   │
│  - Recebe requisição com arquivo                                │
│  - Processa (envia email com anexo)                             │
│  - Retorna confirmação                                          │
└─────────────────────────────────────────────────────────────────┘
```

## 🔄 Fluxo de Upload Detalhado

```
┌──────────────┐
│ Arquivo Local│
│ report.pdf   │
└──────┬───────┘
       │
       │ 1. FileUploadable::from_path()
       ▼
┌──────────────────────────────────────┐
│ Validações                            │
│ ✓ Arquivo existe?                    │
│ ✓ É um arquivo (não diretório)?     │
│ ✓ Tem permissão de leitura?         │
│ ✓ Calcula MD5 hash                  │
│ ✓ Detecta MIME type                 │
└──────┬───────────────────────────────┘
       │
       │ 2. Request presigned URL
       ▼
┌──────────────────────────────────────┐
│ POST /api/v3/files/upload/request    │
│ {                                     │
│   "md5": "abc123...",                │
│   "filename": "report.pdf",          │
│   "mimetype": "application/pdf",     │
│   "tool_slug": "GMAIL_SEND_EMAIL",  │
│   "toolkit_slug": "gmail"            │
│ }                                     │
└──────┬───────────────────────────────┘
       │
       │ 3. Resposta da API
       ▼
┌──────────────────────────────────────┐
│ Response                              │
│ {                                     │
│   "id": "file_xyz",                  │
│   "key": "s3://bucket/path/file",   │
│   "type": "application/pdf",         │
│   "new_presigned_url": "https://..." │
│ }                                     │
└──────┬───────────────────────────────┘
       │
       │ 4. Upload para S3
       ▼
┌──────────────────────────────────────┐
│ PUT https://s3.amazonaws.com/...     │
│ Content-Type: application/pdf        │
│ Body: [binary file content]          │
└──────┬───────────────────────────────┘
       │
       │ 5. Retorno
       ▼
┌──────────────────────────────────────┐
│ FileUploadable {                      │
│   name: "report.pdf",                │
│   mimetype: "application/pdf",       │
│   s3key: "s3://bucket/path/file"    │
│ }                                     │
└──────────────────────────────────────┘
```

## 🔽 Fluxo de Download Detalhado

```
┌──────────────────────────────────────┐
│ Tool Response                         │
│ {                                     │
│   "success": true,                   │
│   "receipt": {                       │
│     "name": "receipt.pdf",           │
│     "mimetype": "application/pdf",   │
│     "s3url": "https://s3.../file"   │
│   }                                   │
│ }                                     │
└──────┬───────────────────────────────┘
       │
       │ 1. SDK detecta file_downloadable
       ▼
┌──────────────────────────────────────┐
│ FileDownloadable::download()          │
│ - Cria diretório de saída            │
│ - Faz GET request para s3url         │
│ - Valida status HTTP                 │
│ - Salva arquivo localmente           │
└──────┬───────────────────────────────┘
       │
       │ 2. Download de S3
       ▼
┌──────────────────────────────────────┐
│ GET https://s3.amazonaws.com/...     │
│ Response: [binary file content]      │
└──────┬───────────────────────────────┘
       │
       │ 3. Salvar localmente
       ▼
┌──────────────────────────────────────┐
│ ~/.composio/outputs/                 │
│   gmail/                              │
│     GMAIL_SEND_EMAIL/                │
│       receipt.pdf                    │
└──────────────────────────────────────┘
```

## 🛡️ Recursos de Segurança

### 1. Validação de Arquivos Locais

```rust
// Verificações antes do upload
✓ Arquivo existe
✓ É um arquivo regular (não link simbólico)
✓ Tem permissão de leitura
✓ Tamanho dentro do limite
```

### 2. Proteções para URLs Públicas

```rust
// Proteções ao baixar de URLs
✓ Apenas HTTP/HTTPS permitido
✓ Redirects desabilitados (previne ataques)
✓ Timeout de conexão: 5 segundos
✓ Timeout de leitura: 60 segundos
✓ Limite de tamanho: 100 MB
✓ Validação de Content-Type
```

### 3. Deduplicação com MD5

```rust
// Evita uploads duplicados
1. Calcula MD5 do arquivo
2. Envia hash para API
3. API verifica se arquivo já existe
4. Se existe, retorna chave existente
5. Se não existe, faz upload
```

### 4. URLs Temporárias

```
- Presigned URLs expiram após uso
- Não expõem credenciais AWS
- Acesso controlado por tempo
```

## 📊 Processamento de Schemas

### Schema Original (Complexo)

```json
{
  "properties": {
    "attachment": {
      "type": "object",
      "file_uploadable": true,
      "properties": {
        "name": {"type": "string"},
        "mimetype": {"type": "string"},
        "s3key": {"type": "string"}
      },
      "required": ["name", "mimetype", "s3key"]
    }
  }
}
```

### Schema Processado (Simples)

```json
{
  "properties": {
    "attachment": {
      "type": "string",
      "format": "path",
      "description": "Path to file. Please provide a value of type string.",
      "file_uploadable": true
    }
  }
}
```

### Transformação de Parâmetros

```
Entrada do Agente:
{
  "attachment": "/home/user/report.pdf"
}

        ↓ SDK processa

Enviado para API:
{
  "attachment": {
    "name": "report.pdf",
    "mimetype": "application/pdf",
    "s3key": "s3://composio-bucket/files/abc123"
  }
}
```

## 🎯 Casos de Uso por Toolkit

### 📧 Gmail / Outlook

```
Upload:
- Anexos de email
- Imagens inline
- Documentos

Download:
- Anexos recebidos
- Backups de emails
```

### 💬 Slack / Discord / WhatsApp

```
Upload:
- Imagens
- Vídeos
- Documentos
- GIFs

Download:
- Mídia compartilhada
- Arquivos de canais
```

### 🗂️ Google Drive / Dropbox

```
Upload:
- Qualquer tipo de arquivo
- Backups
- Sincronização

Download:
- Arquivos compartilhados
- Backups restaurados
```

### 📊 Processamento de Documentos

```
Upload:
- PDFs para análise
- Imagens para OCR
- Planilhas para processamento

Download:
- Relatórios gerados
- Dados extraídos
- Resultados de análise
```

### 🎨 Geração de Conteúdo (DALL-E, Midjourney)

```
Upload:
- Templates
- Referências
- Assets

Download:
- Imagens geradas
- Vídeos renderizados
- Designs criados
```

## 💾 Estrutura de Diretórios

```
~/.composio/                    # Diretório raiz
├── outputs/                    # Arquivos baixados
│   ├── gmail/
│   │   ├── GMAIL_SEND_EMAIL/
│   │   │   ├── receipt_001.pdf
│   │   │   └── receipt_002.pdf
│   │   └── GMAIL_GET_ATTACHMENT/
│   │       ├── document.docx
│   │       └── image.png
│   ├── slack/
│   │   └── SLACK_SEND_MESSAGE/
│   │       ├── screenshot.png
│   │       └── video.mp4
│   └── github/
│       └── GITHUB_CREATE_RELEASE/
│           └── release_notes.md
└── cache/                      # Cache temporário
    └── uploads/
        ├── temp_file_001.tmp
        └── temp_file_002.tmp
```

## 🔧 Configuração

### Variáveis de Ambiente

```bash
# Diretório de cache customizado
export COMPOSIO_CACHE_DIR="/custom/path/.composio"

# API Key
export COMPOSIO_API_KEY="your-api-key"
```

### Limites Configuráveis

```rust
// Constantes que podem ser ajustadas
const MAX_RESPONSE_SIZE: usize = 100 * 1024 * 1024;  // 100 MB
const MAX_FILENAME_LENGTH: usize = 100;
const CONNECT_TIMEOUT_SECS: u64 = 5;
const READ_TIMEOUT_SECS: u64 = 60;
const DEFAULT_CHUNK_SIZE: usize = 1024 * 1024;  // 1 MB
```

## 📈 Métricas e Monitoramento

### Informações Rastreadas

```
- Tamanho do arquivo
- Tempo de upload
- Tempo de download
- MD5 hash (deduplicação)
- Tool slug (qual ferramenta usou)
- Toolkit slug (qual serviço)
- MIME type
- Timestamp de criação
```

### Logs Importantes

```
[INFO] Uploading file: report.pdf (2.5 MB)
[INFO] MD5 hash: abc123...
[INFO] Requesting presigned URL...
[INFO] Uploading to S3...
[INFO] Upload complete: s3://bucket/path/file
[INFO] File available for tool: GMAIL_SEND_EMAIL
```

## 🚀 Performance

### Otimizações Implementadas

1. **Streaming de Arquivos**
   - Leitura em chunks de 1 MB
   - Não carrega arquivo inteiro na memória

2. **Deduplicação**
   - MD5 hash evita uploads duplicados
   - Economiza banda e tempo

3. **Async/Await**
   - Operações não bloqueantes
   - Múltiplos uploads simultâneos

4. **Timeouts Inteligentes**
   - Connect timeout curto (5s)
   - Read timeout longo (60s)
   - Previne travamentos

## 🔮 Futuras Melhorias

1. **Compressão Automática**
   - Comprimir arquivos grandes antes do upload
   - Descomprimir automaticamente no download

2. **Retry com Backoff**
   - Retry automático em falhas temporárias
   - Exponential backoff

3. **Progress Callbacks**
   - Notificar progresso de upload/download
   - Útil para arquivos grandes

4. **Cache Local**
   - Cache de arquivos baixados
   - Evita downloads duplicados

5. **Streaming de Vídeo**
   - Suporte para streaming de arquivos grandes
   - Útil para vídeos e áudio
