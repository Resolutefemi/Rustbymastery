-- Challenges
CREATE TABLE IF NOT EXISTS challenges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rank_requirement TEXT,
    title TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL,
    starter_code TEXT NOT NULL,
    test_cases JSONB NOT NULL DEFAULT '[]',
    difficulty TEXT NOT NULL, -- easy, medium, hard, insane
    xp_reward INTEGER NOT NULL,
    ren_reward INTEGER NOT NULL,
    is_daily BOOLEAN NOT NULL DEFAULT FALSE,
    is_weekly BOOLEAN NOT NULL DEFAULT FALSE,
    is_monthly BOOLEAN NOT NULL DEFAULT FALSE,
    challenge_date DATE,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Challenge Leaderboard Entries
CREATE TABLE IF NOT EXISTS challenge_leaderboard_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    challenge_id UUID NOT NULL REFERENCES challenges(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    submission_id UUID, -- reference to submission if stored
    rank_position INTEGER,
    execution_time_ms INTEGER,
    solved_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
