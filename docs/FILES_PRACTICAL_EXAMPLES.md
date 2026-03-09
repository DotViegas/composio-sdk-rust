# Exemplos Práticos do Módulo de Arquivos

## 📚 Índice

1. [Enviar Email com Anexo](#1-enviar-email-com-anexo)
2. [Enviar Mensagem no Slack com Imagem](#2-enviar-mensagem-no-slack-com-imagem)
3. [Upload para Google Drive](#3-upload-para-google-drive)
4. [Processar Documento PDF](#4-processar-documento-pdf)
5. [Gerar Imagem com IA](#5-gerar-imagem-com-ia)
6. [Backup de Arquivos](#6-backup-de-arquivos)
7. [Conversão de Formatos](#7-conversão-de-formatos)

---

## 1. Enviar Email com Anexo

### Cenário
Você quer que um agente de IA envie um relatório mensal por email com um PDF anexado.

### Código

```rust
use composio_sdk::{
    client::ComposioClient,
    session::SessionBuilder,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Inicializar cliente
    let client = ComposioClient::builder()
        .api_key(&std::env::var("COMPOSIO_API_KEY")?)
        .build()?;
    
    // 2. Criar sessão para o usuário
    let session = client
        .create_session("user_123")
        .toolkits(&["gmail"])
        .build()
        .await?;
    
    // 3. Executar ferramenta com arquivo
    // O SDK automaticamente faz upload do arquivo para S3
    let result = session.execute_tool(
        "GMAIL_SEND_EMAIL",
        serde_json::json!({
            "to": "cliente@empresa.com",
            "subject": "Relatório Mensal - Janeiro 2024",
            "body": "Segue em anexo o relatório mensal.",
            "attachments": [
                "/home/user/relatorios/janeiro_2024.pdf"
            ]
        })
    ).await?;
    
    println!("Email enviado: {:?}", result);
    
    Ok(())
}
```

### O que acontece internamente:

1. SDK detecta que `attachments` é um campo `file_uploadable`
2. Lê o arquivo `/home/user/relatorios/janeiro_2024.pdf`
3. Calcula MD5 hash
4. Faz upload para S3
5. Substitui o caminho pela estrutura S3:
   ```json
   {
     "name": "janeiro_2024.pdf",
     "mimetype": "application/pdf",
     "s3key": "s3://composio/files/abc123"
   }
   ```
6. Envia para Gmail API
7. Gmail baixa de S3 e anexa ao email

---

## 2. Enviar Mensagem no Slack com Imagem

### Cenário
Enviar uma screenshot para um canal do Slack.

### Código

```rust
use composio_sdk::client::ComposioClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::builder()
        .api_key(&std::env::var("COMPOSIO_API_KEY")?)
        .build()?;
    
    let session = client
        .create_session("user_456")
        .toolkits(&["slack"])
        .build()
        .await?;
    
    // Enviar imagem de URL pública
    let result = session.execute_tool(
        "SLACK_SEND_MESSAGE",
        serde_json::json!({
            "channel": "#general",
            "text": "Confira esta screenshot!",
            "files": [
                "https://example.com/screenshot.png"
            ]
        })
    ).await?;
    
    println!("Mensagem enviada: {:?}", result);
    
    Ok(())
}
```

### Suporte para URLs

O SDK automaticamente:
1. Detecta que é uma URL (começa com `http://` ou `https://`)
2. Baixa o arquivo da URL
3. Faz upload para S3
4. Envia para Slack API

---

## 3. Upload para Google Drive

### Cenário
Fazer backup de múltiplos arquivos para o Google Drive.

### Código

```rust
use composio_sdk::client::ComposioClient;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::builder()
        .api_key(&std::env::var("COMPOSIO_API_KEY")?)
        .build()?;
    
    let session = client
        .create_session("user_789")
        .toolkits(&["googledrive"])
        .build()
        .await?;
    
    // Lista de arquivos para backup
    let files = vec![
        "/home/user/documentos/contrato.pdf",
        "/home/user/documentos/proposta.docx",
        "/home/user/documentos/planilha.xlsx",
    ];
    
    // Upload de cada arquivo
    for file_path in files {
        let result = session.execute_tool(
            "GOOGLEDRIVE_UPLOAD_FILE",
            serde_json::json!({
                "file": file_path,
                "folder_id": "1ABC123XYZ",  // ID da pasta no Drive
                "name": PathBuf::from(file_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            })
        ).await?;
        
        println!("Arquivo enviado: {} -> {:?}", file_path, result);
    }
    
    Ok(())
}
```

---

## 4. Processar Documento PDF

### Cenário
Enviar um PDF para análise e receber um relatório extraído.

### Código

```rust
use composio_sdk::{
    client::ComposioClient,
    models::files::FileDownloadable,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::builder()
        .api_key(&std::env::var("COMPOSIO_API_KEY")?)
        .build()?;
    
    let session = client
        .create_session("user_101")
        .toolkits(&["documentai"])
        .build()
        .await?;
    
    // 1. Upload e processamento
    let result = session.execute_tool(
        "DOCUMENTAI_EXTRACT_TEXT",
        serde_json::json!({
            "document": "/home/user/invoice.pdf",
            "extract_tables": true,
            "extract_images": true
        })
    ).await?;
    
    println!("Processamento completo!");
    
    // 2. Download do resultado (se a ferramenta retornar arquivo)
    if let Some(output_file) = result.get("output_file") {
        let downloadable: FileDownloadable = serde_json::from_value(
            output_file.clone()
        )?;
        
        let local_path = downloadable.download(
            Path::new("/home/user/downloads")
        ).await?;
        
        println!("Resultado salvo em: {}", local_path.display());
    }
    
    Ok(())
}
```

---

## 5. Gerar Imagem com IA

### Cenário
Gerar uma imagem usando DALL-E e salvar localmente.

### Código

```rust
use composio_sdk::{
    client::ComposioClient,
    models::files::FileDownloadable,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::builder()
        .api_key(&std::env::var("COMPOSIO_API_KEY")?)
        .build()?;
    
    let session = client
        .create_session("user_202")
        .toolkits(&["openai"])
        .build()
        .await?;
    
    // 1. Gerar imagem
    let result = session.execute_tool(
        "OPENAI_GENERATE_IMAGE",
        serde_json::json!({
            "prompt": "A futuristic city with flying cars at sunset",
            "size": "1024x1024",
            "quality": "hd"
        })
    ).await?;
    
    // 2. Download da imagem gerada
    if let Some(image_data) = result.get("image") {
        let downloadable: FileDownloadable = serde_json::from_value(
            image_data.clone()
        )?;
        
        let local_path = downloadable.download(
            Path::new("/home/user/images")
        ).await?;
        
        println!("Imagem salva em: {}", local_path.display());
    }
    
    Ok(())
}
```

---

## 6. Backup de Arquivos

### Cenário
Sistema de backup automático que envia arquivos para múltiplos destinos.

### Código

```rust
use composio_sdk::client::ComposioClient;
use std::path::{Path, PathBuf};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::builder()
        .api_key(&std::env::var("COMPOSIO_API_KEY")?)
        .build()?;
    
    // Criar sessões para diferentes serviços
    let gdrive_session = client
        .create_session("user_backup")
        .toolkits(&["googledrive"])
        .build()
        .await?;
    
    let dropbox_session = client
        .create_session("user_backup")
        .toolkits(&["dropbox"])
        .build()
        .await?;
    
    // Diretório para backup
    let backup_dir = Path::new("/home/user/important_docs");
    
    // Listar arquivos
    let mut entries = fs::read_dir(backup_dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        
        if path.is_file() {
            let file_path = path.to_str().unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            
            println!("Fazendo backup de: {}", file_name);
            
            // Backup para Google Drive
            let gdrive_result = gdrive_session.execute_tool(
                "GOOGLEDRIVE_UPLOAD_FILE",
                serde_json::json!({
                    "file": file_path,
                    "name": file_name
                })
            ).await;
            
            match gdrive_result {
                Ok(_) => println!("  ✓ Google Drive"),
                Err(e) => println!("  ✗ Google Drive: {}", e),
            }
            
            // Backup para Dropbox
            let dropbox_result = dropbox_session.execute_tool(
                "DROPBOX_UPLOAD_FILE",
                serde_json::json!({
                    "file": file_path,
                    "path": format!("/backups/{}", file_name)
                })
            ).await;
            
            match dropbox_result {
                Ok(_) => println!("  ✓ Dropbox"),
                Err(e) => println!("  ✗ Dropbox: {}", e),
            }
        }
    }
    
    println!("Backup completo!");
    
    Ok(())
}
```

---

## 7. Conversão de Formatos

### Cenário
Converter documentos entre diferentes formatos (PDF → Word, Imagem → PDF, etc.).

### Código

```rust
use composio_sdk::{
    client::ComposioClient,
    models::files::FileDownloadable,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::builder()
        .api_key(&std::env::var("COMPOSIO_API_KEY")?)
        .build()?;
    
    let session = client
        .create_session("user_convert")
        .toolkits(&["cloudconvert"])
        .build()
        .await?;
    
    // Converter PDF para Word
    let result = session.execute_tool(
        "CLOUDCONVERT_CONVERT_FILE",
        serde_json::json!({
            "input_file": "/home/user/document.pdf",
            "output_format": "docx",
            "options": {
                "quality": "high"
            }
        })
    ).await?;
    
    // Download do arquivo convertido
    if let Some(converted_file) = result.get("output_file") {
        let downloadable: FileDownloadable = serde_json::from_value(
            converted_file.clone()
        )?;
        
        let local_path = downloadable.download(
            Path::new("/home/user/converted")
        ).await?;
        
        println!("Arquivo convertido salvo em: {}", local_path.display());
    }
    
    Ok(())
}
```

---

## 🔧 Uso Avançado

### Upload Manual com FileUploadable

Se você precisa de mais controle sobre o processo de upload:

```rust
use composio_sdk::{
    client::ComposioClient,
    models::files::FileUploadable,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ComposioClient::builder()
        .api_key(&std::env::var("COMPOSIO_API_KEY")?)
        .build()?;
    
    // Upload manual
    let uploadable = FileUploadable::from_path(
        &client,
        Path::new("/home/user/report.pdf"),
        "GMAIL_SEND_EMAIL",
        "gmail"
    ).await?;
    
    println!("Arquivo enviado!");
    println!("  Nome: {}", uploadable.name);
    println!("  MIME: {}", uploadable.mimetype);
    println!("  S3 Key: {}", uploadable.s3key);
    
    // Agora você pode usar o uploadable em múltiplas ferramentas
    let session = client
        .create_session("user_123")
        .toolkits(&["gmail"])
        .build()
        .await?;
    
    // Usar o mesmo arquivo em múltiplos emails
    for recipient in &["user1@example.com", "user2@example.com"] {
        session.execute_tool(
            "GMAIL_SEND_EMAIL",
            serde_json::json!({
                "to": recipient,
                "subject": "Relatório",
                "body": "Segue anexo",
                "attachments": [uploadable.clone()]
            })
        ).await?;
    }
    
    Ok(())
}
```

### Processamento de Schema Customizado

```rust
use composio_sdk::models::files::FileHelper;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let helper = FileHelper::new(None);
    
    // Schema original da ferramenta
    let schema = json!({
        "type": "object",
        "properties": {
            "document": {
                "type": "object",
                "file_uploadable": true
            },
            "options": {
                "type": "object",
                "properties": {
                    "quality": {"type": "string"}
                }
            }
        },
        "required": ["document"]
    });
    
    // Processar schema
    let processed = helper.process_schema_recursively(schema);
    
    println!("Schema processado:");
    println!("{}", serde_json::to_string_pretty(&processed)?);
    
    Ok(())
}
```

---

## 🎯 Melhores Práticas

### 1. Validar Arquivos Antes do Upload

```rust
use std::path::Path;
use std::fs;

fn validate_file(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Arquivo não existe: {}", path.display()));
    }
    
    if !path.is_file() {
        return Err(format!("Não é um arquivo: {}", path.display()));
    }
    
    let metadata = fs::metadata(path)
        .map_err(|e| format!("Erro ao ler metadados: {}", e))?;
    
    let size = metadata.len();
    if size > 100 * 1024 * 1024 {  // 100 MB
        return Err(format!("Arquivo muito grande: {} MB", size / 1024 / 1024));
    }
    
    Ok(())
}
```

### 2. Tratamento de Erros

```rust
match FileUploadable::from_path(&client, path, tool, toolkit).await {
    Ok(uploadable) => {
        println!("Upload bem-sucedido: {}", uploadable.s3key);
    }
    Err(e) => {
        eprintln!("Erro no upload: {}", e);
        
        // Tratamento específico por tipo de erro
        match e {
            ComposioError::FileNotFound(_) => {
                eprintln!("Arquivo não encontrado. Verifique o caminho.");
            }
            ComposioError::FileTooLarge(_) => {
                eprintln!("Arquivo muito grande. Máximo: 100 MB");
            }
            ComposioError::UploadFailed(_) => {
                eprintln!("Falha no upload. Tente novamente.");
            }
            _ => {
                eprintln!("Erro desconhecido.");
            }
        }
    }
}
```

### 3. Progress Tracking (Futuro)

```rust
// Exemplo de como poderia ser no futuro
let uploadable = FileUploadable::from_path_with_progress(
    &client,
    path,
    tool,
    toolkit,
    |progress| {
        println!("Upload: {}%", progress.percentage());
    }
).await?;
```

### 4. Limpeza de Arquivos Temporários

```rust
use std::path::Path;
use tokio::fs;

async fn cleanup_temp_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = Path::new("/tmp/composio_uploads");
    
    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir).await?;
        println!("Arquivos temporários removidos");
    }
    
    Ok(())
}
```

---

## 🐛 Troubleshooting

### Erro: "File not found"

```rust
// Verificar se o arquivo existe
use std::path::Path;

let path = Path::new("/home/user/file.pdf");
if !path.exists() {
    eprintln!("Arquivo não existe: {}", path.display());
    eprintln!("Caminho absoluto: {}", path.canonicalize()?.display());
}
```

### Erro: "Upload failed"

```rust
// Verificar permissões
use std::fs;

let metadata = fs::metadata(path)?;
let permissions = metadata.permissions();

if permissions.readonly() {
    eprintln!("Arquivo é somente leitura!");
}
```

### Erro: "File too large"

```rust
// Verificar tamanho
let size = fs::metadata(path)?.len();
let size_mb = size / 1024 / 1024;

println!("Tamanho do arquivo: {} MB", size_mb);

if size_mb > 100 {
    eprintln!("Arquivo muito grande! Máximo: 100 MB");
}
```

---

## 📚 Recursos Adicionais

- [Documentação da API de Files](../COMPOSIO%20DOCS/13-API-REFERENCE-COMPOSIO-FILES.md)
- [Arquitetura do Sistema](FILES_ARCHITECTURE.md)
- [Implementação Detalhada](FILES_IMPLEMENTATION.md)
- [Exemplo Completo de Workflow](../examples/file_workflow_explained.rs)
