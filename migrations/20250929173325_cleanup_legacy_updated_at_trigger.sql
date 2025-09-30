-- Add migration script here
-- Drop the old trigger/function if they exist (from earlier schema)
DO $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM pg_trigger t
    JOIN pg_class c ON c.oid = t.tgrelid
    WHERE c.relname = 'todos' AND t.tgname = 'trg_todos_updated_at'
  ) THEN
    EXECUTE 'DROP TRIGGER trg_todos_updated_at ON public.todos';
  END IF;
END $$;

DO $$
BEGIN
  IF EXISTS (SELECT 1 FROM pg_proc WHERE proname = 'set_updated_at') THEN
    EXECUTE 'DROP FUNCTION public.set_updated_at()';
  END IF;
END $$;
