import { useState, type FormEvent } from "react";
import { Link, useNavigate } from "react-router-dom";
import { Activity } from "lucide-react";
import { useAuth } from "@/lib/auth";
import { Button } from "@/components/Button";
import { Input } from "@/components/Input";
import { ApiError } from "@/lib/api";

export default function SignupPage() {
  const [form, setForm] = useState({
    firstname: "",
    lastname: "",
    username: "",
    password: "",
  });
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const { signup, signin } = useAuth();
  const navigate = useNavigate();

  const set = (field: keyof typeof form) => (e: React.ChangeEvent<HTMLInputElement>) =>
    setForm((prev) => ({ ...prev, [field]: e.target.value }));

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      await signup(form);
      await signin(form.username, form.password);
      navigate("/dashboard");
    } catch (err) {
      if (err instanceof ApiError && err.status === 409) {
        setError("Username already taken. Please choose another.");
      } else {
        setError("Something went wrong. Please try again.");
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-slate-950 px-4">
      <div className="w-full max-w-sm">
        <div className="mb-8 flex flex-col items-center gap-3">
          <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-indigo-600">
            <Activity className="h-6 w-6 text-white" />
          </div>
          <div className="text-center">
            <h1 className="text-2xl font-bold text-slate-100">Create an account</h1>
            <p className="mt-1 text-sm text-slate-400">Start monitoring your endpoints</p>
          </div>
        </div>

        <div className="rounded-xl border border-slate-800 bg-slate-900 p-6 shadow-xl">
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="grid grid-cols-2 gap-3">
              <Input
                label="First name"
                type="text"
                placeholder="John"
                value={form.firstname}
                onChange={set("firstname")}
                required
              />
              <Input
                label="Last name"
                type="text"
                placeholder="Doe"
                value={form.lastname}
                onChange={set("lastname")}
                required
              />
            </div>
            <Input
              label="Username"
              type="text"
              placeholder="john_doe"
              value={form.username}
              onChange={set("username")}
              required
              autoComplete="username"
            />
            <Input
              label="Password"
              type="password"
              placeholder="••••••••"
              value={form.password}
              onChange={set("password")}
              required
              autoComplete="new-password"
              minLength={8}
            />
            {error && (
              <p className="rounded-lg bg-red-500/10 border border-red-500/20 px-3 py-2 text-sm text-red-400">
                {error}
              </p>
            )}
            <Button type="submit" loading={loading} className="w-full" size="lg">
              Create account
            </Button>
          </form>
        </div>

        <p className="mt-5 text-center text-sm text-slate-500">
          Already have an account?{" "}
          <Link to="/login" className="text-indigo-400 hover:text-indigo-300 font-medium">
            Sign in
          </Link>
        </p>
      </div>
    </div>
  );
}
