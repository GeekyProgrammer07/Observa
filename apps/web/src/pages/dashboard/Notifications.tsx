import { useEffect, useState, type FormEvent } from "react";
import { Bell, Plus, Trash2, CheckCircle2, AlertCircle } from "lucide-react";
import { notifications as notificationsApi, ApiError } from "@/lib/api";
import type { NotificationChannel } from "@/types";
import { Button } from "@/components/Button";
import { Modal } from "@/components/Modal";
import { Input } from "@/components/Input";

const channelTypes = ["Email", "Webhook"] as const;

const defaultForm = { channel_type: "Email" as "Email" | "Webhook", value: "" };

export default function NotificationsPage() {
  const [channels, setChannels] = useState<NotificationChannel[]>([]);
  const [loading, setLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);
  const [form, setForm] = useState(defaultForm);
  const [formError, setFormError] = useState("");
  const [creating, setCreating] = useState(false);
  const [verifying, setVerifying] = useState<string | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<string | null>(null);

  const fetchChannels = () =>
    notificationsApi.list().then(setChannels).finally(() => setLoading(false));

  useEffect(() => {
    fetchChannels();
  }, []);

  const handleCreate = async (e: FormEvent) => {
    e.preventDefault();
    setFormError("");
    setCreating(true);
    try {
      await notificationsApi.create(form);
      setShowModal(false);
      setForm(defaultForm);
      fetchChannels();
    } catch (err) {
      if (err instanceof ApiError && err.status === 409) {
        setFormError("This channel already exists.");
      } else {
        setFormError("Failed to add channel. Please try again.");
      }
    } finally {
      setCreating(false);
    }
  };

  const handleVerify = async (id: string) => {
    setVerifying(id);
    try {
      await notificationsApi.verify(id);
      setChannels((prev) =>
        prev.map((c) => (c.id === id ? { ...c, verified: true } : c))
      );
    } finally {
      setVerifying(null);
    }
  };

  const confirmDelete = async () => {
    if (!deleteTarget) return;
    await notificationsApi.delete(deleteTarget);
    setChannels((prev) => prev.filter((c) => c.id !== deleteTarget));
    setDeleteTarget(null);
  };

  const valuePlaceholder =
    form.channel_type === "Email" ? "alerts@example.com" : "https://hooks.example.com/...";
  const valueLabel = form.channel_type === "Email" ? "Email address" : "Webhook URL";

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-slate-100">Notification channels</h1>
          <p className="mt-1 text-sm text-slate-400">
            Configure where alerts are sent when a monitor goes down.
          </p>
        </div>
        <Button onClick={() => setShowModal(true)}>
          <Plus className="h-4 w-4" />
          Add channel
        </Button>
      </div>

      <div className="rounded-xl border border-slate-800 bg-slate-900">
        {loading ? (
          <div className="flex items-center justify-center py-20">
            <div className="h-8 w-8 animate-spin rounded-full border-2 border-indigo-500 border-t-transparent" />
          </div>
        ) : channels.length === 0 ? (
          <div className="flex flex-col items-center gap-3 py-16 text-center">
            <div className="flex h-14 w-14 items-center justify-center rounded-xl bg-slate-800">
              <Bell className="h-6 w-6 text-slate-500" />
            </div>
            <div>
              <p className="font-medium text-slate-300">No channels configured</p>
              <p className="mt-0.5 text-sm text-slate-500">
                Add an email or webhook to receive downtime alerts.
              </p>
            </div>
            <Button onClick={() => setShowModal(true)} className="mt-2">
              <Plus className="h-4 w-4" />
              Add your first channel
            </Button>
          </div>
        ) : (
          <ul className="divide-y divide-slate-800">
            {channels.map((c) => (
              <li key={c.id} className="flex items-center gap-4 px-5 py-4">
                <div
                  className={`flex h-9 w-9 shrink-0 items-center justify-center rounded-lg ${
                    c.channel_type === "Email"
                      ? "bg-indigo-500/20 text-indigo-400"
                      : "bg-violet-500/20 text-violet-400"
                  }`}
                >
                  <Bell className="h-4 w-4" />
                </div>

                <div className="min-w-0 flex-1">
                  <p className="truncate text-sm font-medium text-slate-100">{c.value}</p>
                  <p className="text-xs text-slate-500">{c.channel_type}</p>
                </div>

                <div className="flex items-center gap-2 shrink-0">
                  {c.verified ? (
                    <span className="flex items-center gap-1.5 rounded-full bg-emerald-500/10 px-2.5 py-0.5 text-xs font-medium text-emerald-400">
                      <CheckCircle2 className="h-3 w-3" />
                      Verified
                    </span>
                  ) : (
                    <Button
                      variant="ghost"
                      size="sm"
                      loading={verifying === c.id}
                      onClick={() => handleVerify(c.id)}
                      className="text-amber-400 hover:text-amber-300"
                    >
                      <AlertCircle className="h-3.5 w-3.5" />
                      Verify
                    </Button>
                  )}
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => setDeleteTarget(c.id)}
                    title="Delete"
                    className="hover:text-red-400"
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              </li>
            ))}
          </ul>
        )}
      </div>

      {/* Create modal */}
      <Modal
        open={showModal}
        title="Add notification channel"
        onClose={() => {
          setShowModal(false);
          setForm(defaultForm);
          setFormError("");
        }}
      >
        <form onSubmit={handleCreate} className="space-y-4">
          <div className="flex flex-col gap-1.5">
            <label className="text-sm font-medium text-slate-300">Channel type</label>
            <div className="flex gap-2">
              {channelTypes.map((type) => (
                <button
                  key={type}
                  type="button"
                  onClick={() => setForm((prev) => ({ ...prev, channel_type: type, value: "" }))}
                  className={`flex-1 rounded-lg border py-2 text-sm font-medium transition-colors ${
                    form.channel_type === type
                      ? "border-indigo-500 bg-indigo-600/20 text-indigo-400"
                      : "border-slate-600 bg-slate-900 text-slate-400 hover:border-slate-500"
                  }`}
                >
                  {type}
                </button>
              ))}
            </div>
          </div>

          <Input
            label={valueLabel}
            type={form.channel_type === "Email" ? "email" : "url"}
            placeholder={valuePlaceholder}
            value={form.value}
            onChange={(e) => setForm((prev) => ({ ...prev, value: e.target.value }))}
            required
          />

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
              Add channel
            </Button>
          </div>
        </form>
      </Modal>

      {/* Delete confirm modal */}
      <Modal
        open={!!deleteTarget}
        title="Remove channel"
        onClose={() => setDeleteTarget(null)}
      >
        <p className="text-sm text-slate-400">
          Are you sure you want to remove this notification channel?
        </p>
        <div className="mt-5 flex justify-end gap-3">
          <Button variant="secondary" onClick={() => setDeleteTarget(null)}>
            Cancel
          </Button>
          <Button variant="danger" onClick={confirmDelete}>
            Remove
          </Button>
        </div>
      </Modal>
    </div>
  );
}
