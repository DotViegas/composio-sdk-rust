# Implementação do Módulo de Arquivos (Files)

## Resumo

Implementação completa do módulo de gerenciamento de arquivos do SDK Composio em Rust, baseado no arquivo Python `temp/composio/core/models/_files.py`.

## Arquivos Criados

### 1. `src/models/files.rs`

Módulo principal com toda a funcionalidade de gerenciamento de arquivos:

#### Estruturas de Dados

- **`FileUploadResponse`**: Resposta da API ao solicitar upload
  - `id`: ID do arquivo
  - `key`: Chave S3
  - `file_type`: Tipo do arquivo
  - `new_presigned_url`: URL pré-assinada para upload

- **`FileUploadable`**: Arquivo que pode ser enviado para S3
  - `name`: Nome do arquivo
  - `mimetype`: Tipo MIME
  - `s3key`: Chave S3 após upload

- **`FileDownloadable`**: Arquivo que pode ser baixado de S3
  - `name`: Nome do arquivo
  - `mimetype`: Tipo MIME
  - `s3url`: URL S3 para download

- **`FileHelper`**: Helper para processamento de schemas com arquivos
  - Processa schemas com propriedades `file_uploadable` e `file_downloadable`
  - Adiciona hints de tipo e notas de campos obrigatórios

#### Funcionalidades Implementadas

##### Upload de Arquivos

1. **`FileUploadable::from_path()`**
   - Upload de arquivos locais ou URLs públicas
   - Validação de existência e permissões
   - Cálculo de hash MD5 para integridade
   - Detecção automática de MIME type

2. **`FileUploadable::from_url()`**
   - Fetch de arquivos de URLs públicas
   - Proteções de segurança:
     - Limite de tamanho (100 MB padrão)
     - Desabilita redirects
     - Timeouts separados (connect: 5s, read: 60s)
   - Truncamento de nomes de arquivo longos
   - Geração de nomes com timestamp

##### Download de Arquivos

1. **`FileDownloadable::download()`**
   - Download de arquivos de S3 para diretório local
   - Criação automática de diretórios
   - Tratamento de erros HTTP

##### Processamento de Schemas

1. **`FileHelper::process_file_uploadable_schema()`**
   - Transforma campos `file_uploadable` para formato de path
   - Suporta anyOf, oneOf, allOf
   - Processa propriedades aninhadas e arrays

2. **`FileHelper::enhance_schema_descriptions()`**
   - Adiciona hints de tipo ("Please provide a value of type...")
   - Adiciona notas de campos obrigatórios
   - Melhora descrições de parâmetros

3. **`FileHelper::process_schema_recursively()`**
   - Combina processamento de arquivos e melhorias de descrição
   - Mantém compatibilidade com código existente

#### Funções Auxiliares

- **`calculate_md5()`**: Calcula hash MD5 de arquivo
- **`is_url()`**: Verifica se string é URL HTTP/HTTPS
- **`get_local_cache_directory()`**: Retorna diretório de cache local
- **`get_local_output_directory()`**: Retorna diretório de saída

#### Constantes

- `DEFAULT_CHUNK_SIZE`: 1 MB
- `FILE_UPLOAD_ENDPOINT`: "/api/v3/files/upload/request"
- `MAX_FILENAME_LENGTH`: 100 caracteres
- `MAX_RESPONSE_SIZE`: 100 MB
- `CONNECT_TIMEOUT_SECS`: 5 segundos
- `READ_TIMEOUT_SECS`: 60 segundos
- `LOCAL_CACHE_DIRECTORY_NAME`: ".composio"
- `ENV_LOCAL_CACHE_DIRECTORY`: "COMPOSIO_CACHE_DIR"

### 2. `examples/file_handling.rs`

Exemplo completo demonstrando:
- Upload de arquivo local
- Upload de arquivo de URL
- Download de arquivo de S3
- Processamento de schema com parâmetros de arquivo

### 3. Atualizações em Arquivos Existentes

#### `src/models/mod.rs`
- Adicionado módulo `files`
- Exportados tipos públicos: `FileUploadable`, `FileDownloadable`, `FileHelper`, `FileUploadResponse`

#### `src/error.rs`
- Adicionados novos tipos de erro:
  - `FileNotFound`: Arquivo não encontrado
  - `InvalidFile`: Arquivo inválido
  - `UploadFailed`: Falha no upload
  - `DownloadFailed`: Falha no download
  - `FileTooLarge`: Arquivo muito grande
  - `IoError`: Erro de I/O

#### `Cargo.toml`
- Adicionadas dependências:
  - `md5 = "0.7"`: Cálculo de hash MD5
  - `mime_guess = "2.0"`: Detecção de MIME type
  - `url = "2.5"`: Parsing de URLs
  - `urlencoding = "2.1"`: Decodificação de URLs
  - `chrono = "0.4"`: Manipulação de datas
  - `uuid = "1.0"`: Geração de UUIDs
  - `dirs = "5.0"`: Diretórios do sistema

## Diferenças em Relação ao Python

### Melhorias

1. **Type Safety**: Rust garante type safety em tempo de compilação
2. **Error Handling**: Uso de `Result<T, E>` ao invés de exceções
3. **Async/Await**: Implementação nativa com Tokio
4. **Memory Safety**: Sem garbage collector, gerenciamento explícito de memória

### Funcionalidades Não Implementadas (ainda)

1. **Substituição de uploads em requests**: 
   - `substitute_file_uploads()` - requer acesso à estrutura `Tool`
   - Será implementado quando integrado com o sistema de execução de ferramentas

2. **Substituição de downloads em responses**:
   - `substitute_file_downloads()` - requer acesso à estrutura `Tool`
   - Será implementado quando integrado com o sistema de execução de ferramentas

3. **Logging detalhado**:
   - Python usa `WithLogger` mixin
   - Rust pode usar `tracing` crate (já disponível como feature opcional)

## Uso

### Upload de Arquivo Local

```rust
use composio_sdk::models::files::FileUploadable;
use std::path::Path;

let uploadable = FileUploadable::from_path(
    &client,
    Path::new("document.pdf"),
    "GMAIL_SEND_EMAIL",
    "gmail"
).await?;

println!("Uploaded: {}", uploadable.s3key);
```

### Upload de URL

```rust
let uploadable = FileUploadable::from_url(
    &client,
    "https://example.com/file.pdf",
    "SLACK_SEND_MESSAGE",
    "slack"
).await?;
```

### Download de Arquivo

```rust
use composio_sdk::models::files::FileDownloadable;

let downloadable = FileDownloadable {
    name: "report.pdf".to_string(),
    mimetype: "application/pdf".to_string(),
    s3url: "https://s3.amazonaws.com/...".to_string(),
};

let path = downloadable.download(Path::new("/tmp/downloads")).await?;
```

### Processamento de Schema

```rust
use composio_sdk::models::files::FileHelper;

let helper = FileHelper::new(None);
let processed_schema = helper.process_schema_recursively(schema);
```

## Testes

Para testar a implementação:

```bash
# Verificar compilação
cargo check --all-features

# Executar exemplo
export COMPOSIO_API_KEY="your-api-key"
cargo run --example file_handling

# Executar testes (quando implementados)
cargo test files
```

## Próximos Passos

1. **Integração com Tool Execution**:
   - Implementar `substitute_file_uploads()` para processar parâmetros de ferramentas
   - Implementar `substitute_file_downloads()` para processar respostas de ferramentas

2. **Testes Unitários**:
   - Testes para upload/download
   - Testes para processamento de schemas
   - Testes para funções auxiliares
   - Mocks para requisições HTTP

3. **Testes de Integração**:
   - Testes com API real (usando wiremock)
   - Testes de upload/download end-to-end

4. **Documentação**:
   - Adicionar mais exemplos
   - Documentar casos de uso comuns
   - Adicionar troubleshooting guide

5. **Otimizações**:
   - Streaming de arquivos grandes
   - Pool de conexões HTTP
   - Cache de metadados

## Compatibilidade

- ✅ Rust 1.70+
- ✅ Tokio async runtime
- ✅ Compatível com API Composio v3
- ✅ Suporta Windows, Linux, macOS

## Referências

- Arquivo Python original: `temp/composio/core/models/_files.py`
- Documentação Composio: File 13 (API Reference - Files)
- Rust async book: https://rust-lang.github.io/async-book/
