//! Evaluación local de controles en una configuración sintética.

/// Configuración mínima usada exclusivamente en ejercicios defensivos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Configuration {
    tls_enabled: bool,
    debug_enabled: bool,
    admin_restricted: bool,
}

impl Configuration {
    /// Construye una configuración local sin tocar servicios ni archivos.
    pub const fn new(tls_enabled: bool, debug_enabled: bool, admin_restricted: bool) -> Self {
        Self {
            tls_enabled,
            debug_enabled,
            admin_restricted,
        }
    }
}

/// Control que necesita revisión humana en el escenario sintético.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Finding {
    TlsRequired,
    DebugDisabled,
    AdminRestricted,
}

/// Devuelve los controles ausentes sin efectuar cambios.
pub fn evaluate(config: Configuration) -> Vec<Finding> {
    let mut findings = Vec::new();
    if !config.tls_enabled {
        findings.push(Finding::TlsRequired);
    }
    if config.debug_enabled {
        findings.push(Finding::DebugDisabled);
    }
    if !config.admin_restricted {
        findings.push(Finding::AdminRestricted);
    }
    findings
}
