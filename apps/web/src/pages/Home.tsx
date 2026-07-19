import { Header } from "@/components/Header";
import { Footer } from "@/components/Footer";
import { FeaturesSection, HeroSection } from "@/features/home";

export default function HomePage() {
  return (
    <div className="flex min-h-screen flex-col bg-slate-950">
      <Header />
      <main className="flex-1">
        <HeroSection />
        <FeaturesSection />
      </main>
      <Footer />
    </div>
  );
}
