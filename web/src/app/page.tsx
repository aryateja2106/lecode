import Header from "@/components/Header";
import Hero from "@/components/sections/Hero";
import Problem from "@/components/sections/Problem";
import Comparison from "@/components/sections/Comparison";
import HowItWorks from "@/components/sections/HowItWorks";
import LeCoder from "@/components/sections/LeCoder";
import Architecture from "@/components/sections/Architecture";
import Pricing from "@/components/sections/Pricing";
import Waitlist from "@/components/sections/Waitlist";
import Footer from "@/components/Footer";

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
