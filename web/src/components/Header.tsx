"use client";

import Link from "next/link";
import { useEffect, useState } from "react";

export default function Header() {
  const [scrolled, setScrolled] = useState(false);

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 24);
    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <header
      style={{
        position: "fixed",
        top: 0,
        left: 0,
        right: 0,
        zIndex: 50,
        transition: "background 0.2s, border-color 0.2s",
        background: scrolled ? "rgba(9,9,11,0.85)" : "transparent",
        backdropFilter: scrolled ? "blur(12px)" : "none",
        WebkitBackdropFilter: scrolled ? "blur(12px)" : "none",
        borderBottom: scrolled ? "1px solid rgba(39,39,42,0.8)" : "1px solid transparent",
      }}
    >
      <div
        style={{
          maxWidth: "1120px",
          margin: "0 auto",
          padding: "0 24px",
          height: "60px",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
        }}
      >
        {/* Logo */}
        <Link
          href="/"
          style={{
            fontSize: "18px",
            fontWeight: "700",
            color: "#FAFAFA",
            textDecoration: "none",
            letterSpacing: "-0.02em",
          }}
        >
          LeSearch
        </Link>

        {/* Nav */}
        <nav
          style={{
            display: "flex",
            alignItems: "center",
            gap: "32px",
          }}
        >
          <a
            href="#features"
            style={{
              fontSize: "14px",
              color: "#A1A1AA",
              textDecoration: "none",
              transition: "color 0.15s",
            }}
            onMouseEnter={(e) => ((e.target as HTMLElement).style.color = "#FAFAFA")}
            onMouseLeave={(e) => ((e.target as HTMLElement).style.color = "#A1A1AA")}
          >
            Features
          </a>
          <a
            href="#lecoder"
            style={{
              fontSize: "14px",
              color: "#A1A1AA",
              textDecoration: "none",
              transition: "color 0.15s",
            }}
            onMouseEnter={(e) => ((e.target as HTMLElement).style.color = "#FAFAFA")}
            onMouseLeave={(e) => ((e.target as HTMLElement).style.color = "#A1A1AA")}
          >
            LeCoder
          </a>
          <a
            href="#"
            style={{
              fontSize: "14px",
              color: "#A1A1AA",
              textDecoration: "none",
              transition: "color 0.15s",
            }}
            onMouseEnter={(e) => ((e.target as HTMLElement).style.color = "#FAFAFA")}
            onMouseLeave={(e) => ((e.target as HTMLElement).style.color = "#A1A1AA")}
          >
            Docs
          </a>
          <a
            href="https://github.com/aryateja2106/lecoder-mconnect"
            target="_blank"
            rel="noopener noreferrer"
            style={{
              fontSize: "14px",
              color: "#A1A1AA",
              textDecoration: "none",
              transition: "color 0.15s",
            }}
            onMouseEnter={(e) => ((e.target as HTMLElement).style.color = "#FAFAFA")}
            onMouseLeave={(e) => ((e.target as HTMLElement).style.color = "#A1A1AA")}
          >
            GitHub
          </a>
        </nav>

        {/* CTA */}
        <a
          href="#waitlist"
          style={{
            display: "inline-flex",
            alignItems: "center",
            padding: "8px 18px",
            fontSize: "14px",
            fontWeight: "600",
            color: "#09090B",
            background: "#FAFAFA",
            borderRadius: "8px",
            textDecoration: "none",
            transition: "opacity 0.15s",
            letterSpacing: "-0.01em",
          }}
          onMouseEnter={(e) => ((e.target as HTMLElement).style.opacity = "0.88")}
          onMouseLeave={(e) => ((e.target as HTMLElement).style.opacity = "1")}
        >
          Get Started
        </a>
      </div>
    </header>
  );
}
