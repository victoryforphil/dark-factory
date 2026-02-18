use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Json,
    Toml,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum IncludeLevel {
    Minimal,
    Full,
}

#[derive(Debug, Parser)]
#[command(name = "dark_cli", about = "Dark Factory CLI frontend")]
pub struct Cli {
    #[arg(
        long,
        env = "DARK_CORE_BASE_URL",
        default_value = "http://localhost:4150"
    )]
    pub base_url: String,

    #[arg(long, value_enum, env = "DARK_CLI_FORMAT", default_value_t = OutputFormat::Pretty)]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init { path: Option<String> },
    Info { path: Option<String> },
    Service(ServiceCommand),
    System(SystemCommand),
    Products(ProductsCommand),
    Variants(VariantsCommand),
    Actors(ActorsCommand),
}

#[derive(Debug, Args)]
pub struct ServiceCommand {
    #[command(subcommand)]
    pub action: ServiceAction,
}

#[derive(Debug, Subcommand)]
pub enum ServiceAction {
    Status,
}

#[derive(Debug, Args)]
pub struct SystemCommand {
    #[command(subcommand)]
    pub action: SystemAction,
}

#[derive(Debug, Subcommand)]
pub enum SystemAction {
    Health,
    Info,
    Metrics,
    Providers,
    ResetDb,
}

#[derive(Debug, Args)]
pub struct ProductsCommand {
    #[command(subcommand)]
    pub action: ProductsAction,
}

#[derive(Debug, Subcommand)]
pub enum ProductsAction {
    List {
        #[arg(long)]
        cursor: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long, value_enum)]
        include: Option<IncludeLevel>,
    },
    Create {
        #[arg(long)]
        locator: String,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long)]
        workspace_locator: Option<String>,
    },
    Get {
        #[arg(long)]
        id: String,
        #[arg(long, value_enum)]
        include: Option<IncludeLevel>,
    },
    Update {
        #[arg(long)]
        id: String,
        #[arg(long)]
        locator: Option<String>,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long)]
        workspace_locator: Option<String>,
    },
    Delete {
        #[arg(long)]
        id: String,
    },
    Clone {
        #[arg(long)]
        product_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        target_path: Option<String>,
        #[arg(long)]
        branch_name: Option<String>,
        #[arg(long)]
        clone_type: Option<String>,
        #[arg(long)]
        source_variant_id: Option<String>,
    },
}

#[derive(Debug, Args)]
pub struct VariantsCommand {
    #[command(subcommand)]
    pub action: VariantsAction,
}

#[derive(Debug, Subcommand)]
pub enum VariantsAction {
    List {
        #[arg(long)]
        cursor: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        product_id: Option<String>,
        #[arg(long)]
        locator: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, default_value_t = true)]
        poll: bool,
    },
    Create {
        #[arg(long)]
        locator: String,
        #[arg(long)]
        product_id: String,
        #[arg(long)]
        name: Option<String>,
    },
    Get {
        #[arg(long)]
        id: String,
        #[arg(long, default_value_t = true)]
        poll: bool,
    },
    Poll {
        #[arg(long)]
        id: String,
        #[arg(long, default_value_t = true)]
        poll: bool,
    },
    ImportActors {
        #[arg(long)]
        id: String,
        #[arg(long)]
        provider: Option<String>,
    },
    Update {
        #[arg(long)]
        id: String,
        #[arg(long)]
        locator: Option<String>,
        #[arg(long)]
        name: Option<String>,
    },
    Delete {
        #[arg(long)]
        id: String,
    },
    Branch {
        #[arg(long)]
        id: String,
        #[arg(long)]
        branch_name: String,
    },
}

#[derive(Debug, Args)]
pub struct ActorsCommand {
    #[command(subcommand)]
    pub action: ActorsAction,
}

#[derive(Debug, Subcommand)]
pub enum ActorsAction {
    List {
        #[arg(long)]
        cursor: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        variant_id: Option<String>,
        #[arg(long)]
        product_id: Option<String>,
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    Create {
        #[arg(long)]
        variant_id: String,
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    Get {
        #[arg(long)]
        id: String,
    },
    Update {
        #[arg(long)]
        id: String,
        #[arg(long)]
        variant_id: Option<String>,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    Delete {
        #[arg(long)]
        id: String,
        #[arg(long, default_value_t = false)]
        terminate: bool,
    },
    Poll {
        #[arg(long)]
        id: String,
    },
    Attach {
        #[arg(long)]
        id: String,
        #[arg(long)]
        model: Option<String>,
        #[arg(long)]
        agent: Option<String>,
    },
    Messages {
        #[command(subcommand)]
        action: ActorMessagesAction,
    },
    Commands {
        #[arg(long)]
        id: String,
        #[arg(long)]
        command: String,
        #[arg(long)]
        args: Option<String>,
        #[arg(long)]
        model: Option<String>,
        #[arg(long)]
        agent: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum ActorMessagesAction {
    Send {
        #[arg(long)]
        id: String,
        #[arg(long)]
        prompt: String,
        #[arg(long)]
        no_reply: bool,
        #[arg(long)]
        model: Option<String>,
        #[arg(long)]
        agent: Option<String>,
    },
    List {
        #[arg(long)]
        id: String,
        #[arg(long)]
        n_last_messages: Option<u32>,
    },
}
