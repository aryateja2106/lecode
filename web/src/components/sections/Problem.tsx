"use client";

import FadeIn from "@/components/FadeIn";

const problems = [
  {
    number: "01",
    title: "Security is an afterthought",
    body: "Most developers run agents with full system access, no isolation, and no monitoring. One compromised session means your entire machine is exposed.",
  },
  {
    number: "02",
    title: "Locked to one device",
    body: "Your agent is running on your laptop. You close the lid. Work stops. There is no way to check in, course-correct, or hand off without being at your desk.",
  },
  {
    number: "03",
    title: "No unified view",
    body: "Five agents, three machines, zero visibility into what is happening where. You are context-switching between SSH sessions, log files, and Slack threads.",
  },
];

export default function Problem() {
  return (
    <section
      id="features"
      style={{
        padding: "112px 24px",
        maxWidth: "1120px",
        margin: "0 auto",
      }}
    >
      <FadeIn>
        <p
          style={{
            fontSize: "12px",
            fontWeight: "500",
            color: "#52525B",
            letterSpacing: "0.1em",
            textTransform: "uppercase",
            marginBottom: "16px",
          }}
        >
          The Problem
        </p>
        <h2
          style={{
            fontSize: "clamp(32px, 4.5vw, 52px)",
            fontWeight: "800",
            letterSpacing: "-0.035em",
            lineHeight: "1.08",
            color: "#FAFAFA",
            margin: "0 0 72px",
            maxWidth: "480px",
          }}
        >
          The Agent Setup Problem
        </h2>
      </FadeIn>

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))",
          gap: "1px",
          border: "1px solid #27272A",
          borderRadius: "16px",
          overflow: "hidden",
        }}
      >
        {problems.map((p, i) => (
          <FadeIn key={p.number} delay={i * 80}>
            <div
              style={{
                padding: "40px 36px",
                background: "#0D0D0F",
                height: "100%",
              }}
            >
              <div
                style={{
                  fontSize: "12px",
                  fontWeight: "500",
                  color: "#3F3F46",
                  letterSpacing: "0.06em",
                  marginBottom: "20px",
                  fontFamily:
                    "'SF Mono', 'Fira Code', 'Fira Mono', 'Roboto Mono', monospace",
                }}
              >
                {p.number}
              </div>
              <h3
                style={{
                  fontSize: "18px",
                  fontWeight: "700",
                  color: "#FAFAFA",
                  letterSpacing: "-0.02em",
                  lineHeight: "1.3",
                  margin: "0 0 16px",
                }}
              >
                {p.title}
              </h3>
              <p
                style={{
                  fontSize: "14px",
                  lineHeight: "1.7",
                  color: "#71717A",
                  margin: "0",
                }}
              >
                {p.body}
              </p>
            </div>
          </FadeIn>
        ))}
      </div>
    </section>
  );
}
