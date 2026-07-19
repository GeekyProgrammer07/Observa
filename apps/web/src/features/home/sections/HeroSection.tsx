import { Link } from "react-router-dom";
import { Activity, ArrowRight } from "lucide-react";

export default function HeroSection() {
  return (
    <section className="relative flex flex-col items-center justify-center overflow-hidden bg-slate-950 px-6 py-32 text-center">
      {/* Grid background */}
      <div
        className="pointer-events-none absolute inset-0 opacity-20"
        style={{
          backgroundImage:
            "linear-gradient(rgba(99,102,241,0.15) 1px, transparent 1px), linear-gradient(90deg, rgba(99,102,241,0.15) 1px, transparent 1px)",
          backgroundSize: "60px 60px",
        }}
      />
      <div className="pointer-events-none absolute inset-0 bg-gradient-to-b from-transparent via-slate-950/80 to-slate-950" />

      <div className="relative z-10 max-w-3xl">
        <div className="mb-6 inline-flex items-center gap-2 rounded-full border border-indigo-500/30 bg-indigo-500/10 px-4 py-1.5 text-sm text-indigo-400">
          <Activity className="h-3.5 w-3.5" />
          Always-on uptime monitoring
        </div>

        <h1 className="text-5xl font-bold tracking-tight text-slate-100 sm:text-6xl">
          Know when your{" "}
          <span className="bg-gradient-to-r from-indigo-400 to-violet-400 bg-clip-text text-transparent">
            services go down
          </span>
        </h1>

        <p className="mt-6 text-lg text-slate-400">
          Observa continuously checks your HTTP endpoints and alerts you the moment
          something breaks — so you can fix it before your users notice.
        </p>

        <div className="mt-10 flex flex-col items-center gap-3 sm:flex-row sm:justify-center">
          <Link
            to="/signup"
            className="inline-flex items-center gap-2 rounded-lg bg-indigo-600 px-6 py-3 text-base font-semibold text-white transition-colors hover:bg-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-950"
          >
            Get started free
            <ArrowRight className="h-4 w-4" />
          </Link>
          <Link
            to="/login"
            className="inline-flex items-center gap-2 rounded-lg border border-slate-700 px-6 py-3 text-base font-semibold text-slate-300 transition-colors hover:border-slate-600 hover:text-slate-100"
          >
            Sign in
          </Link>
        </div>
      </div>
    </section>
  );
}
