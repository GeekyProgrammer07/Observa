import { useEffect, useState, type FormEvent } from "react";
import {
  Pause,
  Play,
  Plus,
  Trash2,
  ExternalLink,
  Clock,
  Timer,
} from "lucide-react";
import { monitors as monitorsApi, ApiError } from "@/lib/api";
import type { Monitor } from "@/types";
import { Button } from "@/components/Button";
import { Modal } from "@/components/Modal";
import { Input } from "@/components/Input";

function formatInterval(seconds: number) {
  if (seconds < 60) return `${seconds}s`;
  return `${Math.floor(seconds / 60)}m`;
}

function MonitorRow({
  monitor,
  onPause,
  onResume,
  onDelete,
}: {
  monitor: Monitor;
  onPause: (id: string) => void;
  onResume: (id: string) => void;
  onDelete: (id: string) => void;
}) {
  const [actioning, setActioning] = useState(false);

  const handleToggle = async () => {
    setActioning(true);
    try {
      if (monitor.is_paused) await onResume(monitor.id);
      else await onPause(monitor.id);
    } finally {
      setActioning(false);
    }
  };

  return (
    <li className="flex items-center gap-4 px-5 py-4">
      <span
        className={`h-2.5 w-2.5 shrink-0 rounded-full ${
          monitor.is_paused ? "bg-amber-400" : "bg-emerald-400 shadow-[0_0_6px_#34d399]"
        }`}
      />

      <div className="min-w-0 flex-1">
        <p className="truncate font-medium text-slate-100">
          {monitor.name ?? monitor.url}
        </p>
        <div className="mt-0.5 flex items-center gap-3 text-xs text-slate-500">
          <a
            href={monitor.url}
            target="_blank"
            rel="noreferrer"
            className="flex items-center gap-1 hover:text-slate-300 truncate max-w-[260px]"
          >
            <ExternalLink className="h-3 w-3 shrink-0" />
            {monitor.url}
          </a>
          <span className="flex items-center gap-1 shrink-0">
            <Clock className="h-3 w-3" />
            {formatInterval(monitor.interval)}
          </span>
          <span className="flex items-center gap-1 shrink-0">
            <Timer className="h-3 w-3" />
            {monitor.timeout_ms}ms
          </span>
        </div>
      </div>

      <span
        className={`shrink-0 rounded-full px-2.5 py-0.5 text-xs font-medium ${
          monitor.is_paused
            ? "bg-amber-500/10 text-amber-400"
            : "bg-emerald-500/10 text-emerald-400"
        }`}
      >
        {monitor.is_paused ? "Paused" : "Active"}
      </span>

      <div className="flex items-center gap-2 shrink-0">
        <Button
          variant="ghost"
          size="sm"
          loading={actioning}
          onClick={handleToggle}
          title={monitor.is_paused ? "Resume" : "Pause"}
        >
          {monitor.is_paused ? (
            <Play className="h-4 w-4" />
          ) : (
            <Pause className="h-4 w-4" />
          )}
        </Button>
        <Button
          variant="ghost"
          size="sm"
          onClick={() => onDelete(monitor.id)}
          title="Delete"
          className="hover:text-red-400"
        >
          <Trash2 className="h-4 w-4" />
        </Button>
      </div>
    </li>
  );
}

const defaultForm = {
  name: "",
  url: "",
  interval: "60",
  timeout_ms: "5000",
};

export default function MonitorsPage() {
  const [monitorList, setMonitorList] = useState<Monitor[]>([]);
  const [loading, setLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);
  const [form, setForm] = useState(defaultForm);
  const [formError, setFormError] = useState("");
  const [creating, setCreating] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<string | null>(null);

  const fetchMonitors = () =>
    monitorsApi.list().then(setMonitorList).finally(() => setLoading(false));

  useEffect(() => {
    fetchMonitors();
  }, []);

  const set = (field: keyof typeof defaultForm) =>
    (e: React.ChangeEvent<HTMLInputElement>) =>
      setForm((prev) => ({ ...prev, [field]: e.target.value }));

  const handleCreate = async (e: FormEvent) => {
    e.preventDefault();
    setFormError("");
    setCreating(true);
    try {
      await monitorsApi.create({
        url: form.url,
        name: form.name || undefined,
        interval: parseInt(form.interval) || 60,
        timeout_ms: parseInt(form.timeout_ms) || 5000,
      });
      setShowModal(false);
      setForm(defaultForm);
      fetchMonitors();
    } catch (err) {
      if (err instanceof ApiError && err.status === 409) {
        setFormError("A monitor for this URL already exists.");
      } else {
        setFormError("Failed to create monitor. Please try again.");
      }
    } finally {
      setCreating(false);
    }
  };

  const handlePause = async (id: string) => {
    await monitorsApi.pause(id);
    setMonitorList((prev) =>
      prev.map((m) => (m.id === id ? { ...m, is_paused: true } : m))
    );
  };

  const handleResume = async (id: string) => {
    await monitorsApi.resume(id);
    setMonitorList((prev) =>
      prev.map((m) => (m.id === id ? { ...m, is_paused: false } : m))
    );
  };

  const handleDelete = async (id: string) => {
    setDeleteTarget(id);
  };

  const confirmDelete = async () => {
    if (!deleteTarget) return;
    await monitorsApi.delete(deleteTarget);
    setMonitorList((prev) => prev.filter((m) => m.id !== deleteTarget));
    setDeleteTarget(null);
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-slate-100">Monitors</h1>
          <p className="mt-1 text-sm text-slate-400">
            Manage your URL endpoints being monitored.
          </p>
        </div>
        <Button onClick={() => setShowModal(true)}>
          <Plus className="h-4 w-4" />
          Add monitor
        </Button>
      </div>

      <div className="rounded-xl border border-slate-800 bg-slate-900">
        {loading ? (
          <div className="flex items-center justify-center py-20">
            <div className="h-8 w-8 animate-spin rounded-full border-2 border-indigo-500 border-t-transparent" />
          </div>
        ) : monitorList.length === 0 ? (
          <div className="flex flex-col items-center gap-3 py-16 text-center">
            <div className="flex h-14 w-14 items-center justify-center rounded-xl bg-slate-800">
              <ExternalLink className="h-6 w-6 text-slate-500" />
            </div>
            <div>
              <p className="font-medium text-slate-300">No monitors yet</p>
              <p className="mt-0.5 text-sm text-slate-500">
                Add a URL to start monitoring its uptime.
              </p>
            </div>
            <Button onClick={() => setShowModal(true)} className="mt-2">
              <Plus className="h-4 w-4" />
              Add your first monitor
            </Button>
          </div>
        ) : (
          <ul className="divide-y divide-slate-800">
            {monitorList.map((m) => (
              <MonitorRow
                key={m.id}
                monitor={m}
                onPause={handlePause}
                onResume={handleResume}
                onDelete={handleDelete}
              />
            ))}
          </ul>
        )}
      </div>

      {/* Create modal */}
      <Modal
        open={showModal}
        title="Add monitor"
        onClose={() => {
          setShowModal(false);
          setForm(defaultForm);
          setFormError("");
        }}
      >
        <form onSubmit={handleCreate} className="space-y-4">
          <Input
            label="URL"
            type="url"
            placeholder="https://example.com"
            value={form.url}
            onChange={set("url")}
            required
          />
          <Input
            label="Name (optional)"
            type="text"
            placeholder="My API"
            value={form.name}
            onChange={set("name")}
          />
          <div className="grid grid-cols-2 gap-3">
            <Input
              label="Interval (seconds)"
              type="number"
              placeholder="60"
              min="10"
              value={form.interval}
              onChange={set("interval")}
              required
            />
            <Input
              label="Timeout (ms)"
              type="number"
              placeholder="5000"
              min="100"
              value={form.timeout_ms}
              onChange={set("timeout_ms")}
              required
            />
          </div>
          {formError && (
            <p className="rounded-lg bg-red-500/10 border border-red-500/20 px-3 py-2 text-sm text-red-400">
              {formError}
            </p>
          )}
          <div className="flex justify-end gap-3 pt-1">
            <Button
              type="button"
              variant="secondary"
              onClick={() => {
                setShowModal(false);
                setForm(defaultForm);
                setFormError("");
              }}
            >
              Cancel
            </Button>
            <Button type="submit" loading={creating}>
              Create monitor
            </Button>
          </div>
        </form>
      </Modal>

      {/* Delete confirm modal */}
      <Modal
        open={!!deleteTarget}
        title="Delete monitor"
        onClose={() => setDeleteTarget(null)}
      >
        <p className="text-sm text-slate-400">
          Are you sure you want to delete this monitor? This action cannot be undone.
        </p>
        <div className="mt-5 flex justify-end gap-3">
          <Button variant="secondary" onClick={() => setDeleteTarget(null)}>
            Cancel
          </Button>
          <Button variant="danger" onClick={confirmDelete}>
            Delete
          </Button>
        </div>
      </Modal>
    </div>
  );
}
