// ------------ boot guard ------------
console.log("[app] app.js loaded");

// wait until window.__TAURI__.core is ready (max ~3s)
async function waitForTauri(timeoutMs = 3000) {
  const start = Date.now();
  while (!window.__TAURI__?.core) {
    if (Date.now() - start > timeoutMs) {
      throw new Error(
        "Tauri core not available. Check tauri.conf.json (withGlobalTauri) and CSP."
      );
    }
    await new Promise((r) => setTimeout(r, 50));
  }
  return window.__TAURI__.core.invoke;
}

// ------------ DOM handles ------------
const els = {
  loading: document.getElementById("loading"),
  error: document.getElementById("error"),
  empty: document.getElementById("empty-state"),
  list: document.getElementById("todo-list"),
  total: document.getElementById("total-count"),
  active: document.getElementById("active-count"),
  completed: document.getElementById("completed-count"),
  form: document.getElementById("add-todo-form"),
  title: document.getElementById("todo-title"),
  desc: document.getElementById("todo-description"),
  prio: document.getElementById("todo-priority"),
  sort: document.getElementById("sort-select"),
  filterBtns: document.querySelectorAll(".filter-btn"),
};

// ------------ state ------------
let currentFilter = "all"; // "all" | "active" | "completed"
let currentSort = "created"; // "created" | "priority" | "title"
let cache = []; // last loaded todos
let invoke; // set after waitForTauri()

// ------------ ui helpers ------------
function showError(e) {
  if (!els.error) return;
  els.error.style.display = "block";
  const msg = typeof e === "string" ? e : e?.message ?? String(e);
  els.error.textContent = msg;
  console.error("[UI error]", e);
}
function clearError() {
  if (!els.error) return;
  els.error.style.display = "none";
  els.error.textContent = "";
}
function setLoading(on) {
  if (!els.loading) return;
  els.loading.style.display = on ? "block" : "none";
}
function priorityOrderVal(p) {
  const order = { urgent: 3, high: 2, medium: 1, low: 0 };
  return order[(p || "").toLowerCase()] ?? 0;
}
function fmtDateISO(s) {
  // Always safe to call; returns a readable local string if valid ISO
  try {
    const d = new Date(s);
    if (Number.isNaN(d.getTime())) return "";
    return d.toLocaleString();
  } catch {
    return "";
  }
}

// ------------ render ------------
function render(todos) {
  if (!els.list) return;

  let list = (todos || []).filter((t) => {
    if (currentFilter === "active") return !t.is_completed;
    if (currentFilter === "completed") return t.is_completed;
    return true;
  });

  if (currentSort === "priority") {
    list.sort(
      (a, b) => priorityOrderVal(b.priority) - priorityOrderVal(a.priority)
    );
  } else if (currentSort === "title") {
    list.sort((a, b) => String(a.title).localeCompare(String(b.title)));
  } else {
    // created (default)
    list.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
  }

  if (els.total) els.total.textContent = todos.length;
  if (els.active)
    els.active.textContent = todos.filter((t) => !t.is_completed).length;
  if (els.completed)
    els.completed.textContent = todos.filter((t) => t.is_completed).length;

  els.list.innerHTML = "";
  if (list.length === 0) {
    if (els.empty) els.empty.style.display = "block";
    return;
  }
  if (els.empty) els.empty.style.display = "none";

  const frag = document.createDocumentFragment();
  for (const t of list) {
    const li = document.createElement("li");
    li.className = "todo-item" + (t.is_completed ? " completed" : "");
    li.dataset.id = t.id;

    // Checkbox (toggle)
    const cb = document.createElement("input");
    cb.type = "checkbox";
    cb.className = "todo-checkbox";
    cb.checked = !!t.is_completed;
    cb.dataset.action = "toggle";

    // Main content
    const content = document.createElement("div");
    content.className = "todo-content";

    const title = document.createElement("div");
    title.className = "todo-title";
    title.textContent = t.title ?? "";

    const desc = document.createElement("div");
    desc.className = "todo-description";
    desc.textContent = t.description || "";
    if (!t.description) desc.style.display = "none";

    const meta = document.createElement("div");
    meta.className = "todo-meta";

    const p = (t.priority || "medium").toLowerCase();
    const badge = document.createElement("span");
    badge.className = `priority-badge priority-${p}`;
    badge.textContent = p;

    const created = document.createElement("span");
    created.textContent = fmtDateISO(t.created_at);

    // If you later add `due_at TIMESTAMPTZ`, we display it when present:
    if (t.due_at) {
      const due = document.createElement("span");
      due.className = "due-at";
      due.textContent = `due: ${fmtDateISO(t.due_at)}`;
      meta.appendChild(due);
    }

    meta.appendChild(badge);
    meta.appendChild(created);

    content.appendChild(title);
    content.appendChild(desc);
    content.appendChild(meta);

    // Actions
    const actions = document.createElement("div");
    actions.className = "todo-actions";

    const editBtn = document.createElement("button");
    editBtn.className = "btn btn-edit";
    editBtn.textContent = "Edit";
    editBtn.dataset.action = "edit";

    const delBtn = document.createElement("button");
    delBtn.className = "btn btn-delete";
    delBtn.textContent = "Delete";
    delBtn.dataset.action = "delete";

    actions.appendChild(editBtn);
    actions.appendChild(delBtn);

    li.appendChild(cb);
    li.appendChild(content);
    li.appendChild(actions);

    frag.appendChild(li);
  }
  els.list.appendChild(frag);
}

// ------------ RPC wrappers ------------
async function refresh() {
  setLoading(true);
  clearError();
  try {
    console.log("[rpc] list_todos()");
    const result = await invoke("list_todos");
    console.log("[rpc] list_todos ->", result);
    cache = Array.isArray(result) ? result : [];
    render(cache);
  } catch (e) {
    showError(e);
  } finally {
    setLoading(false);
  }
}

async function createTodoFromForm() {
  clearError();
  const title = (els.title?.value || "").trim();
  const description = (els.desc?.value || "").trim();
  const priority = (els.prio?.value || "medium").toLowerCase();

  if (!title) return showError("Title cannot be empty");

  const payload = {
    title,
    description: description.length ? description : null,
    priority, // "low" | "medium" | "high" | "urgent"
  };

  try {
    console.log("[rpc] create_todo()", payload);
    await invoke("create_todo", { payload });
    els.form?.reset();
    await refresh();
  } catch (e) {
    showError(e);
  }
}

async function toggleTodo(id, toCompleted) {
  try {
    console.log("[rpc] toggle_todo()", { id, toCompleted });
    // IMPORTANT: keep camelCase to match the Tauri command signature when using `#![allow(non_snake_case)]`
    await invoke("toggle_todo", { id, toCompleted });
    await refresh();
  } catch (e) {
    showError(e);
  }
}

async function deleteTodo(id, title) {
  if (!confirm(`Delete "${title}"?`)) return;
  try {
    console.log("[rpc] delete_todo()", { id });
    await invoke("delete_todo", { id });
    await refresh();
  } catch (e) {
    showError(e);
  }
}

async function openEditPrompt(todo) {
  try {
    const newTitle = prompt("Edit title:", todo.title ?? "");
    if (newTitle === null) return;

    const newDesc = prompt(
      "Edit description (leave blank for none):",
      todo.description || ""
    );
    if (newDesc === null) return;

    const newPrio = prompt(
      "Priority (low|medium|high|urgent):",
      (todo.priority || "medium").toLowerCase()
    );
    if (newPrio === null) return;

    const patch = {};
    const titleTrim = (newTitle || "").trim();
    if (titleTrim && titleTrim !== todo.title) patch.title = titleTrim;

    const descNorm = (newDesc || "").trim();
    const origDesc = (todo.description || "").trim();
    if (descNorm !== origDesc)
      patch.description = descNorm.length ? descNorm : null;

    const prioNorm = (newPrio || "").trim().toLowerCase();
    if (prioNorm && prioNorm !== (todo.priority || "").toLowerCase()) {
      patch.priority = prioNorm; // will be validated by backend
    }

    if (Object.keys(patch).length === 0) return;

    console.log("[rpc] update_todo()", { id: todo.id, patch });
    await invoke("update_todo", { id: todo.id, patch });
    await refresh();
  } catch (e) {
    showError(e);
  }
}

// ------------ wiring ------------
function wireEvents() {
  els.filterBtns.forEach((btn) => {
    btn.addEventListener("click", () => {
      els.filterBtns.forEach((b) => b.classList.remove("active"));
      btn.classList.add("active");
      currentFilter = btn.dataset.filter;
      render(cache);
    });
  });

  if (els.sort) {
    els.sort.addEventListener("change", () => {
      currentSort = els.sort.value;
      render(cache);
    });
  }

  if (els.form) {
    els.form.addEventListener("submit", async (e) => {
      e.preventDefault();
      await createTodoFromForm();
    });
  }

  if (els.list) {
    els.list.addEventListener("click", async (e) => {
      const actionEl = e.target.closest?.("[data-action]");
      if (!actionEl) return;

      const li = e.target.closest?.("li.todo-item");
      if (!li) return;

      const id = Number(li.dataset.id);
      const todo = cache.find((t) => t.id === id);
      if (!todo) return;

      const action = actionEl.dataset.action;
      if (action === "edit") {
        await openEditPrompt(todo);
      } else if (action === "delete") {
        await deleteTodo(id, todo.title);
      }
    });

    els.list.addEventListener("change", async (e) => {
      const target = e.target;
      if (!(target instanceof HTMLInputElement)) return;
      if (!target.matches(".todo-checkbox")) return;

      const li = target.closest("li.todo-item");
      if (!li) return;

      const id = Number(li.dataset.id);
      await toggleTodo(id, target.checked);
    });
  }
}

// ------------ boot ------------
document.addEventListener("DOMContentLoaded", async () => {
  try {
    invoke = await waitForTauri(); // get the real invoke AFTER API injection
    console.log("[app] Tauri core ready");
  } catch (e) {
    showError(e);
    return; // don't proceed if API isn't available
  }
  wireEvents();
  refresh();
});
