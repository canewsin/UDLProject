/// Status of an invoice
enum EnumWithDescription {
    /// Invoice not yet sent
    DRAFT,
    SENT,
    OVERDUE,
    CANCELLED,
    /// Payment received
    PAID {
        timestamp: i32,
        trasaction_id: String,
        customer_id: String,
    },
}
