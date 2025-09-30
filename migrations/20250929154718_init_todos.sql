-- Align existing `todos` table to the new schema (idempotent).

-- 1) Ensure enum type exists
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'priority_level') THEN
    CREATE TYPE priority_level AS ENUM ('low', 'medium', 'high', 'urgent');
  END IF;
END $$;

-- 2) Rename legacy column completed -> is_completed if needed
DO $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_schema = 'public' AND table_name = 'todos' AND column_name = 'completed'
  ) AND NOT EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_schema = 'public' AND table_name = 'todos' AND column_name = 'is_completed'
  ) THEN
    EXECUTE 'ALTER TABLE public.todos RENAME COLUMN completed TO is_completed';
  END IF;
END $$;

-- 3) Add/ensure columns with sane defaults
ALTER TABLE public.todos
  ADD COLUMN IF NOT EXISTS title        VARCHAR(255)        NOT NULL DEFAULT '' ,
  ADD COLUMN IF NOT EXISTS description  TEXT,
  ADD COLUMN IF NOT EXISTS is_completed BOOLEAN             NOT NULL DEFAULT FALSE,
  ADD COLUMN IF NOT EXISTS priority     priority_level      NOT NULL DEFAULT 'medium',
  ADD COLUMN IF NOT EXISTS created_at   TIMESTAMPTZ         NOT NULL DEFAULT now(),
  ADD COLUMN IF NOT EXISTS updated_at   TIMESTAMPTZ         NOT NULL DEFAULT now(),
  ADD COLUMN IF NOT EXISTS completed_at TIMESTAMPTZ;

-- Remove empty default on title if we just added it (keep NOT NULL)
ALTER TABLE public.todos ALTER COLUMN title DROP DEFAULT;

-- 4) (Re)create helper functions
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at := now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_completed_at()
RETURNS TRIGGER AS $$
BEGIN
  IF NEW.is_completed = TRUE AND OLD.is_completed = FALSE THEN
    NEW.completed_at := now();
  ELSIF NEW.is_completed = FALSE AND OLD.is_completed = TRUE THEN
    NEW.completed_at := NULL;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 5) Recreate triggers (drop if exist first)
DROP TRIGGER IF EXISTS set_updated_at ON public.todos;
CREATE TRIGGER set_updated_at
  BEFORE UPDATE ON public.todos
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS set_completed_timestamp ON public.todos;
CREATE TRIGGER set_completed_timestamp
  BEFORE UPDATE ON public.todos
  FOR EACH ROW
  EXECUTE FUNCTION set_completed_at();

-- 6) Ensure indexes exist
CREATE INDEX IF NOT EXISTS idx_todos_is_completed ON public.todos(is_completed);
CREATE INDEX IF NOT EXISTS idx_todos_priority     ON public.todos(priority);
CREATE INDEX IF NOT EXISTS idx_todos_created_at   ON public.todos(created_at DESC);
