use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Server};
use tower_lsp::jsonrpc;

#[derive(Debug)]
struct TomLanguageServer;

#[tower_lsp::async_trait]
impl LanguageServer for TomLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        println!("TOM Language Server initialized.");
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> jsonrpc::Result<Option<Hover>> {
        let hover = Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "This is a TOM language hover!".to_string(),
            )),
            range: None,
        };
        Ok(Some(hover))
    }
}

#[tokio::main]
async fn main() {
    println!("Initializing TOM Language Server...");
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|_client| TomLanguageServer).finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}
