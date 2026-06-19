-- WaveFlow migration 002: webhook delivery id deduplication index
CREATE UNIQUE INDEX IF NOT EXISTS idx_webhook_events_delivery_id
    ON webhook_events (delivery_id)
    WHERE delivery_id IS NOT NULL;
