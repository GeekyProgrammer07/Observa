import { NavLink, useNavigate } from "react-router-dom";
import { Activity, Bell, LayoutDashboard, LogOut } from "lucide-react";
import { useAuth } from "@/lib/auth";

const navItems = [
  { to: "/dashboard", label: "Overview", icon: LayoutDashboard, end: true },
  { to: "/dashboard/monitors", label: "Monitors", icon: Activity, end: false },
  { to: "/dashboard/notifications", label: "Notifications", icon: Bell, end: false },
];

export function DashboardLayout({ children }: { children: React.ReactNode }) {
  const { signout } = useAuth();
  const navigate = useNavigate();

  const handleSignout = () => {
    signout();
    navigate("/login");
  };

  return (
    <div className="flex h-screen bg-slate-950 text-slate-100">
      {/* Sidebar */}
      <aside className="flex w-60 flex-col border-r border-slate-800 bg-slate-900">
        <div className="flex items-center gap-2.5 px-5 py-5 border-b border-slate-800">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-indigo-600">
            <Activity className="h-4 w-4 text-white" />
          </div>
          <span className="text-lg font-bold tracking-tight">Observa</span>
        </div>

        <nav className="flex-1 space-y-1 px-3 py-4">
          {navItems.map(({ to, label, icon: Icon, end }) => (
            <NavLink
              key={to}
              to={to}
              end={end}
              className={({ isActive }) =>
                `flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors ${
                  isActive
                    ? "bg-indigo-600/20 text-indigo-400"
                    : "text-slate-400 hover:bg-slate-800 hover:text-slate-100"
                }`
              }
            >
              <Icon className="h-4 w-4 shrink-0" />
              {label}
            </NavLink>
          ))}
        </nav>

        <div className="border-t border-slate-800 p-3">
          <button
            onClick={handleSignout}
            className="flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium text-slate-400 transition-colors hover:bg-slate-800 hover:text-red-400"
          >
            <LogOut className="h-4 w-4 shrink-0" />
            Sign out
          </button>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 overflow-y-auto">
        <div className="mx-auto max-w-5xl px-8 py-8">{children}</div>
      </main>
    </div>
  );
}
