import Footer from "@/components/Footer";
import Header from "@/components/Header";
import Architecture from "@/components/sections/Architecture";
import Comparison from "@/components/sections/Comparison";
import Hero from "@/components/sections/Hero";
import HowItWorks from "@/components/sections/HowItWorks";
import LeCoder from "@/components/sections/LeCoder";
import Pricing from "@/components/sections/Pricing";
import Problem from "@/components/sections/Problem";
import Waitlist from "@/components/sections/Waitlist";

export default function Home() {
  return (
    <>
      <Header />
      <main>
        <Hero />
        <Problem />
        <Comparison />
        <HowItWorks />
        <LeCoder />
        <Architecture />
        <Pricing />
        <Waitlist />
      </main>
      <Footer />
    </>
  );
}
