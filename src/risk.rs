//! Priorización local de riesgos documentados para ejercicios defensivos.

/// Categoría orientativa de una matriz impacto-probabilidad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Riesgo con impacto y probabilidad en la escala documentada de uno a cinco.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Risk {
    impact: u8,
    likelihood: u8,
}

impl Risk {
    /// Construye un riesgo validado; los valores fuera de escala son inválidos.
    pub fn try_new(impact: u8, likelihood: u8) -> Option<Self> {
        ((1..=5).contains(&impact) && (1..=5).contains(&likelihood))
            .then_some(Self { impact, likelihood })
    }

    /// Construye un riesgo dentro de la escala de laboratorio.
    pub fn new(impact: u8, likelihood: u8) -> Self {
        Self::try_new(impact, likelihood).expect("los puntajes deben estar entre uno y cinco")
    }

    /// Devuelve una prioridad para orientar mitigación y revisión humana.
    pub fn priority(self) -> Priority {
        match self.impact * self.likelihood {
            1..=4 => Priority::Low,
            5..=9 => Priority::Medium,
            10..=16 => Priority::High,
            _ => Priority::Critical,
        }
    }
}
