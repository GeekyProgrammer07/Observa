import { Link } from "react-router-dom";
import { Activity } from "lucide-react";
import { useAuth } from "@/lib/auth";

export function Header() {
  const { isAuthenticated } = useAuth();

  return (
    <header className="sticky top-0 z-40 border-b border-slate-800 bg-slate-950/80 backdrop-blur-sm">
      <div className="mx-auto flex max-w-5xl items-center justify-between px-6 py-4">
        <Link to="/" className="flex items-center gap-2 font-bold text-slate-100">
          <div className="flex h-7 w-7 items-center justify-center rounded-lg bg-indigo-600">
            <Activity className="h-4 w-4 text-white" />
          </div>
          Observa
        </Link>

        <nav className="flex items-center gap-3">
          {isAuthenticated ? (
            <Link
              to="/dashboard"
              className="rounded-lg bg-indigo-600 px-4 py-1.5 text-sm font-medium text-white transition-colors hover:bg-indigo-500"
            >
              Dashboard
            </Link>
          ) : (
            <>
              <Link
                to="/login"
                className="text-sm font-medium text-slate-400 transition-colors hover:text-slate-100"
              >
                Sign in
              </Link>
              <Link
                to="/signup"
                className="rounded-lg bg-indigo-600 px-4 py-1.5 text-sm font-medium text-white transition-colors hover:bg-indigo-500"
              >
                Get started
              </Link>
            </>
          )}
        </nav>
      </div>
    </header>
  );
}
