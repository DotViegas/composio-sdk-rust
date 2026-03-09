//! Exemplo explicando o fluxo completo de arquivos no Composio SDK
//!
//! Este exemplo demonstra como arquivos fluem através do sistema:
//! 1. Usuário fornece arquivo local
//! 2. SDK faz upload para S3
//! 3. Ferramenta usa o arquivo
//! 4. Resultado pode conter arquivos para download
//!
//! # Cenário Real
//!
//! Um agente de IA que:
//! - Recebe um documento PDF
//! - Envia por email usando Gmail
//! - Recebe confirmação com anexo de recibo

use composio_sdk::{
    client::ComposioClient,
    models::files::{FileUploadable, FileDownloadable, FileHelper},
};
use std::path::Path;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Fluxo Completo de Arquivos no Composio ===\n");
    
    let api_key = std::env::var("COMPOSIO_API_KEY")
        .unwrap_or_else(|_| "demo-key".to_string());
    
    let client = ComposioClient::builder()
        .api_key(&api_key)
        .build()?;
    
    // ========================================================================
    // ETAPA 1: Usuário tem um arquivo local
    // ========================================================================
    println!("📁 ETAPA 1: Arquivo Local");
    println!("   Usuário: 'Envie este relatório por email'");
    println!("   Arquivo: relatorio_mensal.pdf (no computador local)\n");
    
    // ========================================================================
    // ETAPA 2: SDK processa o schema da ferramenta
    // ========================================================================
    println!("🔧 ETAPA 2: Processamento de Schema");
    
    // Schema original da ferramenta GMAIL_SEND_EMAIL
    let original_schema = json!({
        "type": "object",
        "properties": {
            "to": {
                "type": "string",
                "description": "Email recipient"
            },
            "subject": {
                "type": "string",
                "description": "Email subject"
            },
            "body": {
                "type": "string",
                "description": "Email body"
            },
            "attachments": {
                "type": "array",
                "items": {
                    "type": "object",
                    "file_uploadable": true,
                    "properties": {
                        "name": {"type": "string"},
                        "mimetype": {"type": "string"},
                        "s3key": {"type": "string"}
                    }
                }
            }
        },
        "required": ["to", "subject"]
    });
    
    println!("   Schema Original (complexo):");
    println!("   - attachments requer: name, mimetype, s3key");
    println!("   - Agente de IA não sabe como preencher isso!\n");
    
    // FileHelper transforma o schema
    let helper = FileHelper::new(None);
    let processed_schema = helper.process_schema_recursively(original_schema.clone());
    
    println!("   Schema Processado (simplificado):");
    println!("   - attachments agora aceita: caminho do arquivo");
    println!("   - Agente de IA pode fornecer: '/path/to/relatorio.pdf'");
    println!("   - SDK faz o resto automaticamente!\n");
    
    // ========================================================================
    // ETAPA 3: Agente de IA fornece parâmetros
    // ========================================================================
    println!("🤖 ETAPA 3: Agente de IA Fornece Parâmetros");
    
    let ai_parameters = json!({
        "to": "cliente@empresa.com",
        "subject": "Relatório Mensal",
        "body": "Segue em anexo o relatório mensal.",
        "attachments": [
            "/home/user/documentos/relatorio_mensal.pdf"
        ]
    });
    
    println!("   Parâmetros do Agente:");
    println!("   {}", serde_json::to_string_pretty(&ai_parameters)?);
    println!();
    
    // ========================================================================
    // ETAPA 4: SDK faz upload automático para S3
    // ========================================================================
    println!("☁️  ETAPA 4: Upload Automático para S3");
    println!("   (Esta etapa seria feita automaticamente pelo SDK)");
    println!();
    
    // Simulação do que acontece internamente:
    if Path::new("README.md").exists() {
        println!("   Simulando upload...");
        
        match FileUploadable::from_path(
            &client,
            Path::new("README.md"),
            "GMAIL_SEND_EMAIL",
            "gmail"
        ).await {
            Ok(uploaded) => {
                println!("   ✓ Arquivo enviado para S3!");
                println!("   - Nome: {}", uploaded.name);
                println!("   - MIME: {}", uploaded.mimetype);
                println!("   - S3 Key: {}", uploaded.s3key);
                println!();
                
                // ========================================================================
                // ETAPA 5: SDK transforma parâmetros para a API
                // ========================================================================
                println!("🔄 ETAPA 5: Transformação de Parâmetros");
                
                let transformed_parameters = json!({
                    "to": "cliente@empresa.com",
                    "subject": "Relatório Mensal",
                    "body": "Segue em anexo o relatório mensal.",
                    "attachments": [
                        {
                            "name": uploaded.name,
                            "mimetype": uploaded.mimetype,
                            "s3key": uploaded.s3key
                        }
                    ]
                });
                
                println!("   Parâmetros Transformados (enviados para Gmail API):");
                println!("   {}", serde_json::to_string_pretty(&transformed_parameters)?);
                println!();
            }
            Err(e) => {
                println!("   ✗ Erro no upload: {}", e);
                println!("   (Isso é esperado sem API key válida)");
                println!();
            }
        }
    }
    
    // ========================================================================
    // ETAPA 6: Ferramenta executa e pode retornar arquivos
    // ========================================================================
    println!("📧 ETAPA 6: Execução da Ferramenta");
    println!("   Gmail API:");
    println!("   1. Recebe os parâmetros com s3key");
    println!("   2. Baixa o arquivo de S3");
    println!("   3. Anexa ao email");
    println!("   4. Envia o email");
    println!("   5. Pode retornar um recibo (também como arquivo)");
    println!();
    
    // ========================================================================
    // ETAPA 7: Download de arquivos retornados
    // ========================================================================
    println!("📥 ETAPA 7: Download de Resultados");
    
    // Simulação de resposta da ferramenta
    let tool_response = json!({
        "success": true,
        "message": "Email sent successfully",
        "receipt": {
            "name": "email_receipt.pdf",
            "mimetype": "application/pdf",
            "s3url": "https://s3.amazonaws.com/composio/receipts/abc123.pdf"
        }
    });
    
    println!("   Resposta da Ferramenta:");
    println!("   {}", serde_json::to_string_pretty(&tool_response)?);
    println!();
    
    // SDK pode baixar automaticamente
    println!("   SDK detecta 'file_downloadable' no schema de resposta");
    println!("   e baixa automaticamente para o diretório local:");
    println!("   ~/.composio/outputs/gmail/GMAIL_SEND_EMAIL/email_receipt.pdf");
    println!();
    
    // ========================================================================
    // RESUMO DO FLUXO
    // ========================================================================
    println!("📊 RESUMO DO FLUXO:");
    println!();
    println!("   Arquivo Local");
    println!("        ↓");
    println!("   FileUploadable.from_path()");
    println!("        ↓");
    println!("   Upload para S3 (Composio)");
    println!("        ↓");
    println!("   s3key gerado");
    println!("        ↓");
    println!("   Parâmetros transformados");
    println!("        ↓");
    println!("   API Externa (Gmail) baixa de S3");
    println!("        ↓");
    println!("   Ferramenta executa");
    println!("        ↓");
    println!("   Resposta pode conter arquivos");
    println!("        ↓");
    println!("   FileDownloadable.download()");
    println!("        ↓");
    println!("   Arquivo Local (resultado)");
    println!();
    
    // ========================================================================
    // POR QUE ISSO É IMPORTANTE?
    // ========================================================================
    println!("💡 POR QUE ISSO É IMPORTANTE?");
    println!();
    println!("   1. SIMPLICIDADE PARA O AGENTE DE IA:");
    println!("      - Agente só precisa fornecer caminhos de arquivo");
    println!("      - Não precisa entender S3, uploads, etc.");
    println!();
    println!("   2. SEGURANÇA:");
    println!("      - Arquivos não são enviados diretamente para APIs externas");
    println!("      - Composio gerencia acesso e permissões");
    println!("      - URLs S3 são temporárias e seguras");
    println!();
    println!("   3. COMPATIBILIDADE:");
    println!("      - Funciona com qualquer API externa");
    println!("      - Suporta arquivos locais e URLs públicas");
    println!("      - Gerencia MIME types automaticamente");
    println!();
    println!("   4. DEDUPLICAÇÃO:");
    println!("      - Hash MD5 evita uploads duplicados");
    println!("      - Economiza banda e armazenamento");
    println!();
    println!("   5. RASTREABILIDADE:");
    println!("      - Cada arquivo tem um ID único");
    println!("      - Logs de upload/download");
    println!("      - Associado a tool_slug e toolkit_slug");
    println!();
    
    // ========================================================================
    // CASOS DE USO REAIS
    // ========================================================================
    println!("🎯 CASOS DE USO REAIS:");
    println!();
    println!("   📧 Email com Anexos (Gmail, Outlook):");
    println!("      - Upload: PDF, imagens, documentos");
    println!("      - Download: Anexos recebidos");
    println!();
    println!("   💬 Mensagens com Mídia (Slack, Discord, WhatsApp):");
    println!("      - Upload: Imagens, vídeos, arquivos");
    println!("      - Download: Mídia compartilhada");
    println!();
    println!("   📊 Processamento de Documentos:");
    println!("      - Upload: Documentos para análise");
    println!("      - Download: Relatórios gerados");
    println!();
    println!("   🗂️  Armazenamento em Nuvem (Google Drive, Dropbox):");
    println!("      - Upload: Arquivos para backup");
    println!("      - Download: Arquivos sincronizados");
    println!();
    println!("   🎨 Geração de Conteúdo:");
    println!("      - Upload: Templates, assets");
    println!("      - Download: Imagens, vídeos gerados");
    println!();
    
    println!("=== Fim do Exemplo ===");
    
    Ok(())
}
