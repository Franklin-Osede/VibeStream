use std::time::{Duration, Instant};
use log::info;

#[derive(Default)]
pub struct WalletMetrics {
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub total_amount_transferred: u64,
    pub average_confirmation_time: Duration,
    pub last_transaction_time: Option<Instant>,
}

impl WalletMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_transaction_attempt(&mut self) {
        self.total_transactions += 1;
        self.last_transaction_time = Some(Instant::now());
    }

    pub fn record_transaction_success(&mut self, amount: u64, confirmation_time: Duration) {
        self.successful_transactions += 1;
        self.total_amount_transferred += amount;
        
        // Actualizar tiempo promedio de confirmación
        let total_time = self.average_confirmation_time.as_millis() as u64 * (self.successful_transactions - 1);
        let new_average = (total_time + confirmation_time.as_millis() as u64) / self.successful_transactions;
        self.average_confirmation_time = Duration::from_millis(new_average as u64);

        info!(
            "Métricas de transacción: \n\
            - Tiempo de confirmación: {:?}\n\
            - Promedio de confirmación: {:?}\n\
            - Total transferido: {} SOL\n\
            - Tasa de éxito: {:.2}%",
            confirmation_time,
            self.average_confirmation_time,
            self.total_amount_transferred as f64 / 1_000_000_000.0,
            (self.successful_transactions as f64 / self.total_transactions as f64) * 100.0
        );
    }

    pub fn record_transaction_failure(&mut self) {
        self.failed_transactions += 1;
        
        info!(
            "Estadísticas de fallos: \n\
            - Total de fallos: {}\n\
            - Tasa de fallos: {:.2}%",
            self.failed_transactions,
            (self.failed_transactions as f64 / self.total_transactions as f64) * 100.0
        );
    }

    pub fn get_success_rate(&self) -> f64 {
        if self.total_transactions == 0 {
            return 0.0;
        }
        (self.successful_transactions as f64 / self.total_transactions as f64) * 100.0
    }

    pub fn get_average_confirmation_time(&self) -> Duration {
        self.average_confirmation_time
    }

    pub fn get_total_transferred(&self) -> f64 {
        self.total_amount_transferred as f64 / 1_000_000_000.0
    }
} 