#[derive(poem_openapi::Tags)]
pub enum ApiTags {
    /// System endpoints
    System,
    /// Authentication endpoints
    Auth,
    /// Employee management
    Employees,
    /// Department management
    Departments,
    /// Position management
    Positions,
    /// Software catalog
    Software,
    /// Computers
    Computers,
    /// Consumables
    Consumables,
    /// Equipment
    Equipment,
    /// Licenses
    Licenses,
    /// Network
    Network,
    /// Monitoring
    Monitoring,
    /// Active Directory
    ActiveDirectory,
    /// Requests
    Requests,
    /// Activity log and audit
    ActivityLog,
    /// WireGuard VPN management
    Wireguard,
}
