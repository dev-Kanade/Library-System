-- =============================================
-- 1. groups（利用者グループ + 貸出ルール）
-- =============================================
CREATE TABLE IF NOT EXISTS groups (
    group_id            INTEGER PRIMARY KEY AUTOINCREMENT,
    group_code          TEXT UNIQUE NOT NULL,
    group_name          TEXT NOT NULL,
    max_borrow_limit    INTEGER NOT NULL DEFAULT 5,
    loan_period_days    INTEGER NOT NULL DEFAULT 14,
    renewal_limit       INTEGER NOT NULL DEFAULT 2,
    max_fine_per_day    REAL DEFAULT 0.00,
    can_reserve         BOOLEAN DEFAULT 1,
    can_borrow          BOOLEAN DEFAULT 1,
    max_reserve_limit   INTEGER NOT NULL DEFAULT 3,
    description         TEXT,
    notes               TEXT,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- =============================================
-- 2. authors（著者マスタ）
-- =============================================
CREATE TABLE IF NOT EXISTS authors (
    author_id           INTEGER PRIMARY KEY AUTOINCREMENT,
    author_name         TEXT NOT NULL,
    author_name_kana    TEXT,
    birth_year          INTEGER,
    description         TEXT,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_author_name ON authors(author_name);

-- =============================================
-- 3. categories（分類マスタ）
-- =============================================
CREATE TABLE IF NOT EXISTS categories (
    category_id         INTEGER PRIMARY KEY AUTOINCREMENT,
    category_name       TEXT NOT NULL UNIQUE,
    description         TEXT,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- =============================================
-- 4. genres（ジャンルマスタ）
-- =============================================
CREATE TABLE IF NOT EXISTS genres (
    genre_id            INTEGER PRIMARY KEY AUTOINCREMENT,
    genre_name          TEXT NOT NULL UNIQUE,
    description         TEXT,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- =============================================
-- 5. storage（蔵書 + 書誌情報）
-- =============================================
CREATE TABLE IF NOT EXISTS storage (
    storage_id          INTEGER PRIMARY KEY AUTOINCREMENT,
    accession_number    TEXT UNIQUE NOT NULL,
    isbn                TEXT,
    title               TEXT NOT NULL,
    subtitle            TEXT,
    publisher           TEXT,
    publication_year    INTEGER,
    pages               INTEGER,
    description         TEXT,
    location            TEXT,
    status              TEXT DEFAULT 'available' NOT NULL,
    acquisition_date    DATE,
    price               REAL,
    notes               TEXT,
    category_id         INTEGER,
    genre_id            INTEGER,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES categories(category_id) ON DELETE SET NULL,
    FOREIGN KEY (genre_id) REFERENCES genres(genre_id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_isbn ON storage(isbn);
CREATE INDEX IF NOT EXISTS idx_title ON storage(title);
CREATE INDEX IF NOT EXISTS idx_status ON storage(status);
CREATE INDEX IF NOT EXISTS idx_location ON storage(location);

-- =============================================
-- 6. book_authors（書籍-著者 多対多）
-- =============================================
CREATE TABLE IF NOT EXISTS book_authors (
    storage_id          INTEGER NOT NULL,
    author_id           INTEGER NOT NULL,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (storage_id, author_id),
    FOREIGN KEY (storage_id) REFERENCES storage(storage_id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES authors(author_id) ON DELETE CASCADE
);

-- =============================================
-- 7. users（利用者）
-- =============================================
CREATE TABLE IF NOT EXISTS users (
    user_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    user_number         TEXT UNIQUE NOT NULL,
    name                TEXT NOT NULL,
    name_kana           TEXT,
    birth_date          DATE,
    gender              TEXT DEFAULT 'unknown',
    email               TEXT UNIQUE,
    phone               TEXT,
    address             TEXT,
    postal_code         TEXT,
    group_id            INTEGER NOT NULL,
    is_active           BOOLEAN DEFAULT 1,
    is_lending_prohibited BOOLEAN DEFAULT 0,
    prohibition_reason  TEXT,
    prohibition_start   DATE,
    prohibition_end     DATE,
    membership_date     DATE NOT NULL,
    expiry_date         DATE,
    notes               TEXT,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (group_id) REFERENCES groups(group_id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_group_id ON users(group_id);
CREATE INDEX IF NOT EXISTS idx_name ON users(name);
CREATE INDEX IF NOT EXISTS idx_user_number ON users(user_number);

-- =============================================
-- 8. loans（貸出状況）
-- =============================================
CREATE TABLE IF NOT EXISTS loans (
    loan_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    storage_id          INTEGER NOT NULL,
    user_id             INTEGER NOT NULL,
    loan_date           DATE NOT NULL,
    due_date            DATE NOT NULL,
    return_date         DATE,
    renewed_count       INTEGER DEFAULT 0,
    notes               TEXT,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (storage_id) REFERENCES storage(storage_id) ON DELETE RESTRICT,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_storage_id ON loans(storage_id);
CREATE INDEX IF NOT EXISTS idx_user_id ON loans(user_id);
CREATE INDEX IF NOT EXISTS idx_due_date ON loans(due_date);

-- =============================================
-- 9. reservations（予約状況）
-- =============================================
CREATE TABLE IF NOT EXISTS reservations (
    reservation_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id             INTEGER NOT NULL,
    storage_id          INTEGER NOT NULL,
    reservation_date    DATE NOT NULL,
    expiry_date         DATE NOT NULL,
    status              TEXT DEFAULT 'pending' NOT NULL,
    notes               TEXT,
    notified_at         DATETIME,
    fulfilled_at        DATETIME,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (storage_id) REFERENCES storage(storage_id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_res_user_id ON reservations(user_id);
CREATE INDEX IF NOT EXISTS idx_res_storage_id ON reservations(storage_id);
CREATE INDEX IF NOT EXISTS idx_res_status ON reservations(status);

-- =============================================
-- 10. notifications（通知履歴）
-- =============================================
CREATE TABLE IF NOT EXISTS notifications (
    notification_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id             INTEGER NOT NULL,
    type                TEXT NOT NULL,
    title               TEXT NOT NULL,
    message             TEXT NOT NULL,
    sent_at             DATETIME,
    read_at             DATETIME,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_notif_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notif_type ON notifications(type);

-- =============================================
-- 11. audit_logs（操作履歴）
-- =============================================
CREATE TABLE IF NOT EXISTS audit_logs (
    log_id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id             INTEGER,
    table_name          TEXT NOT NULL,
    record_id           INTEGER,
    action              TEXT NOT NULL,
    old_values          TEXT,
    new_values          TEXT,
    ip_address          TEXT,
    notes               TEXT,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_table_record ON audit_logs(table_name, record_id);
CREATE INDEX IF NOT EXISTS idx_action ON audit_logs(action);