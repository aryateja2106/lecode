"use client";

import { useState } from "react";
import { toast } from "sonner";
import FadeIn from "@/components/FadeIn";
import { joinWaitlist } from "@/lib/supabase";

export default function Waitlist() {
  const [email, setEmail] = useState("");
  const [loading, setLoading] = useState(false);
  const [done, setDone] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!email.trim()) return;

    setLoading(true);
    const result = await joinWaitlist(email.trim().toLowerCase());
    setLoading(false);

    if (result.success) {
      if (result.message === "Already on waitlist") {
        toast.info("Already on the waitlist", {
          description: "We have your email. We will reach out when access opens.",
        });
      } else {
        toast.success("You are on the waitlist", {
          description: "We will reach out when early access opens.",
        });
        setDone(true);
      }
    } else {
      toast.error("Something went wrong", {
        description: result.message,
      });
    }
  }

  return (
    <section
      id="waitlist"
      style={{
        padding: "112px 24px",
        maxWidth: "1120px",
        margin: "0 auto",
      }}
    >
      <FadeIn>
        <div
          style={{
            border: "1px solid #27272A",
            borderRadius: "20px",
            background: "#0D0D0F",
            padding: "clamp(48px, 8vw, 80px) clamp(32px, 6vw, 80px)",
            textAlign: "center",
            position: "relative",
            overflow: "hidden",
          }}
        >
          {/* Top accent line */}
          <div
            style={{
              position: "absolute",
              top: 0,
              left: "50%",
              transform: "translateX(-50%)",
              width: "160px",
              height: "1px",
              background: "linear-gradient(90deg, transparent, #52525B, transparent)",
            }}
          />

          <p
            style={{
              fontSize: "12px",
              fontWeight: "500",
              color: "#52525B",
              letterSpacing: "0.1em",
              textTransform: "uppercase",
              marginBottom: "20px",
            }}
          >
            Early Access
          </p>
          <h2
            style={{
              fontSize: "clamp(32px, 5vw, 56px)",
              fontWeight: "800",
              letterSpacing: "-0.035em",
              lineHeight: "1.08",
              color: "#FAFAFA",
              margin: "0 0 20px",
            }}
          >
            Get Early Access
          </h2>
          <p
            style={{
              fontSize: "17px",
              lineHeight: "1.65",
              color: "#71717A",
              maxWidth: "440px",
              margin: "0 auto 48px",
            }}
          >
            We are onboarding power users who run agents across multiple
            machines. Spots are limited.
          </p>

          {done ? (
            <div
              style={{
                display: "inline-flex",
                alignItems: "center",
                gap: "10px",
                padding: "16px 32px",
                border: "1px solid #27272A",
                borderRadius: "12px",
                background: "#18181B",
              }}
            >
              <div
                style={{
                  width: "8px",
                  height: "8px",
                  borderRadius: "50%",
                  background: "#FAFAFA",
                }}
              />
              <span style={{ fontSize: "15px", fontWeight: "500", color: "#A1A1AA" }}>
                You are on the list. We will be in touch.
              </span>
            </div>
          ) : (
            <form
              onSubmit={handleSubmit}
              style={{
                display: "flex",
                gap: "10px",
                justifyContent: "center",
                flexWrap: "wrap",
                maxWidth: "460px",
                margin: "0 auto",
              }}
            >
              <input
                type="email"
                required
                placeholder="your@email.com"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                disabled={loading}
                style={{
                  flex: "1",
                  minWidth: "200px",
                  padding: "12px 16px",
                  fontSize: "14px",
                  color: "#FAFAFA",
                  background: "#18181B",
                  border: "1px solid #27272A",
                  borderRadius: "10px",
                  outline: "none",
                  transition: "border-color 0.15s",
                }}
                onFocus={(e) =>
                  ((e.target as HTMLInputElement).style.borderColor = "#52525B")
                }
                onBlur={(e) =>
                  ((e.target as HTMLInputElement).style.borderColor = "#27272A")
                }
              />
              <button
                type="submit"
                disabled={loading}
                style={{
                  padding: "12px 24px",
                  fontSize: "14px",
                  fontWeight: "600",
                  color: "#09090B",
                  background: loading ? "#A1A1AA" : "#FAFAFA",
                  border: "none",
                  borderRadius: "10px",
                  cursor: loading ? "not-allowed" : "pointer",
                  whiteSpace: "nowrap",
                  transition: "opacity 0.15s",
                  letterSpacing: "-0.01em",
                }}
                onMouseEnter={(e) => {
                  if (!loading) (e.target as HTMLElement).style.opacity = "0.88";
                }}
                onMouseLeave={(e) => {
                  (e.target as HTMLElement).style.opacity = "1";
                }}
              >
                {loading ? "Joining..." : "Join Waitlist"}
              </button>
            </form>
          )}

          <p style={{ fontSize: "12px", color: "#3F3F46", marginTop: "20px" }}>
            No spam. Unsubscribe anytime.
          </p>
        </div>
      </FadeIn>
    </section>
  );
}
