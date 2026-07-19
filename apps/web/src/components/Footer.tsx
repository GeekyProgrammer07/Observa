import { Activity } from "lucide-react";

export function Footer() {
  return (
    <footer className="border-t border-slate-800 bg-slate-950 px-6 py-8">
      <div className="mx-auto flex max-w-5xl items-center justify-between">
        <div className="flex items-center gap-2 text-sm text-slate-500">
          <Activity className="h-4 w-4" />
          Observa — uptime monitoring
        </div>
        <p className="text-xs text-slate-600">
          Built with Rust + React
        </p>
      </div>
    </footer>
  );
}
