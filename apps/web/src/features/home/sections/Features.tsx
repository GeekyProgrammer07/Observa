import { Activity, Bell, Clock, Globe, Shield, Zap } from "lucide-react";

const features = [
  {
    icon: Globe,
    title: "HTTP endpoint monitoring",
    description:
      "Monitor any public or private URL. Observa sends HTTP checks at your configured interval.",
  },
  {
    icon: Bell,
    title: "Instant alerts",
    description:
      "Get notified via email or webhook the moment a check fails, so you can respond fast.",
  },
  {
    icon: Clock,
    title: "Custom intervals",
    description:
      "Set your own check frequency from seconds to minutes. Monitor critical endpoints more aggressively.",
  },
  {
    icon: Zap,
    title: "Worker queue system",
    description:
      "Powered by a Redis-backed worker queue for reliable, distributed check execution at scale.",
  },
  {
    icon: Shield,
    title: "Pause & resume",
    description:
      "Temporarily pause a monitor during maintenance windows without losing your configuration.",
  },
  {
    icon: Activity,
    title: "Multi-channel notifications",
    description:
      "Configure multiple notification channels per account — email for your team, webhooks for Slack.",
  },
];

export default function FeaturesSection() {
  return (
    <section className="bg-slate-950 px-6 py-24">
      <div className="mx-auto max-w-5xl">
        <div className="text-center">
          <h2 className="text-3xl font-bold text-slate-100">
            Everything you need to stay informed
          </h2>
          <p className="mt-3 text-slate-400">
            A focused set of features built for reliability engineers and developers.
          </p>
        </div>

        <div className="mt-14 grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
          {features.map(({ icon: Icon, title, description }) => (
            <div
              key={title}
              className="rounded-xl border border-slate-800 bg-slate-900 p-6 transition-colors hover:border-slate-700"
            >
              <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-500/20">
                <Icon className="h-5 w-5 text-indigo-400" />
              </div>
              <h3 className="mt-4 font-semibold text-slate-100">{title}</h3>
              <p className="mt-2 text-sm text-slate-400">{description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
