"use client";

import { useRef, useEffect, useState } from "react";

interface FadeInProps {
  children: React.ReactNode;
  delay?: number;
  className?: string;
  style?: React.CSSProperties;
}

/**
 * A scroll-triggered fade-in that:
 * - Renders visible by default (SSR/headless safe)
 * - Adds a subtle animation when JavaScript runs and IntersectionObserver fires
 * - Falls back gracefully when IntersectionObserver is unavailable
 */
export default function FadeIn({
  children,
  delay = 0,
  style,
}: FadeInProps) {
  const ref = useRef<HTMLDivElement>(null);
  const [visible, setVisible] = useState(false);
  const [jsReady, setJsReady] = useState(false);

  useEffect(() => {
    // Mark JS as ready — this hides content briefly before reveal
    // We only do this if the element is NOT already in viewport
    const el = ref.current;
    if (!el) return;

    if (!("IntersectionObserver" in window)) {
      // Fallback: just show everything
      setVisible(true);
      return;
    }

    const rect = el.getBoundingClientRect();
    const alreadyVisible = rect.top < window.innerHeight;

    if (alreadyVisible) {
      // Already in viewport on load — animate in immediately
      setJsReady(true);
      requestAnimationFrame(() => {
        setTimeout(() => setVisible(true), delay);
      });
      return;
    }

    // Below fold — set up observer
    setJsReady(true);
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setTimeout(() => setVisible(true), delay);
          observer.disconnect();
        }
      },
      { threshold: 0.05 }
    );

    observer.observe(el);
    return () => observer.disconnect();
  }, [delay]);

  return (
    <div
      ref={ref}
      style={{
        ...style,
        opacity: jsReady ? (visible ? 1 : 0) : 1,
        transform: jsReady
          ? visible
            ? "translateY(0)"
            : "translateY(20px)"
          : "none",
        transition: jsReady
          ? "opacity 0.55s ease, transform 0.55s ease"
          : "none",
      }}
    >
      {children}
    </div>
  );
}
