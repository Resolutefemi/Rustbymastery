-- Ren Coin Transactions
CREATE TABLE IF NOT EXISTS ren_coin_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount INTEGER NOT NULL, -- positive = earned, negative = spent
    transaction_type TEXT NOT NULL, -- lesson_complete, quiz_pass, challenge_solve, daily_reward, streak_bonus, referral, challenge_prize, purchase, gift, admin_grant
    reference_id UUID, -- ID of lesson/challenge/etc that triggered it
    note TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Referrals
CREATE TABLE IF NOT EXISTS referrals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    referrer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    referred_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status TEXT NOT NULL, -- signed_up, completed_rank1, converted_to_paid
    ren_coins_awarded INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    converted_at TIMESTAMPTZ
);
