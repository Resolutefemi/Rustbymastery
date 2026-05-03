-- Certificates
CREATE TABLE IF NOT EXISTS certificates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    certificate_type TEXT NOT NULL, -- rank, full_course, specialist_track, fast_track
    rank_slug TEXT,
    track_name TEXT,
    full_name_on_cert TEXT NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    blockchain_tx_hash TEXT,
    verification_url TEXT NOT NULL,
    pdf_url TEXT
);

-- Fast Track Exams
CREATE TABLE IF NOT EXISTS fast_track_exams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_rank TEXT NOT NULL,
    status TEXT NOT NULL, -- in_progress, passed, failed, abandoned
    score_quiz INTEGER,
    score_coding INTEGER,
    ren_cost_deducted INTEGER NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);
