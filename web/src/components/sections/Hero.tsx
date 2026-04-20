"use client";

import { motion } from "framer-motion";

const terminalLines = [
  { prompt: true, text: "npx lecoder-mconnect" },
  { prompt: false, text: "" },
  { prompt: false, text: "  LeSearch MConnect v1.0.2", dim: false, bold: true },
  { prompt: false, text: "  Secure agent tunnel initializing...", dim: true },
  { prompt: false, text: "" },
  { prompt: false, text: "  Cloudflare tunnel active", prefix: "[ok]" },
  { prompt: false, text: "  SSH hardening: enabled", prefix: "[ok]" },
  { prompt: false, text: "  Agent isolation: sandbox", prefix: "[ok]" },
  { prompt: false, text: "" },
  { prompt: false, text: "  Scan to connect from any device:", dim: true },
  { prompt: false, text: "" },
  { prompt: false, text: "  QR_CODE", qr: true },
  { prompt: false, text: "" },
  { prompt: false, text: "  mconnect://tun-a1b2c3.lesearch.ai", code: true },
  { prompt: false, text: "" },
  { prompt: false, text: "  Waiting for connection...", dim: true, cursor: true },
];

function QRCode() {
  const size = 8;
  const pattern = [
    [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1],
    [0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0],
    [1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1],
    [0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0],
    [1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1],
    [1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0],
    [1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1],
  ];

  return (
    <div
      style={{
        display: "inline-block",
        background: "#FAFAFA",
        padding: "6px",
        borderRadius: "4px",
      }}
    >
      {pattern.map((row, r) => (
        <div key={r} style={{ display: "flex" }}>
          {row.map((cell, c) => (
            <div
              key={c}
              style={{
                width: `${size}px`,
                height: `${size}px`,
                background: cell ? "#09090B" : "#FAFAFA",
              }}
            />
          ))}
        </div>
      ))}
    </div>
  );
}

export default function Hero() {
  return (
    <section
      style={{
        minHeight: "100vh",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        padding: "120px 24px 80px",
        maxWidth: "1120px",
        margin: "0 auto",
      }}
    >
      {/* Badge */}
      <motion.div
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
        style={{ marginBottom: "32px" }}
      >
        <span
          style={{
            display: "inline-flex",
            alignItems: "center",
            gap: "8px",
            padding: "5px 14px",
            fontSize: "12px",
            fontWeight: "500",
            color: "#A1A1AA",
            border: "1px solid #27272A",
            borderRadius: "100px",
            letterSpacing: "0.04em",
            textTransform: "uppercase",
          }}
        >
          <span
            style={{
              width: "6px",
              height: "6px",
              borderRadius: "50%",
              background: "#FAFAFA",
              display: "inline-block",
            }}
          />
          Early Access Open
        </span>
      </motion.div>

      {/* Headline */}
      <motion.h1
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.55, delay: 0.08 }}
        style={{
          fontSize: "clamp(42px, 7vw, 76px)",
          fontWeight: "800",
          letterSpacing: "-0.04em",
          lineHeight: "1.02",
          textAlign: "center",
          color: "#FAFAFA",
          margin: "0 0 24px",
          maxWidth: "860px",
        }}
      >
        Every Agent.
        <br />
        Every Machine.
        <br />
        One Terminal.
      </motion.h1>

      {/* Sub */}
      <motion.p
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.55, delay: 0.16 }}
        style={{
          fontSize: "18px",
          lineHeight: "1.65",
          color: "#A1A1AA",
          textAlign: "center",
          maxWidth: "560px",
          margin: "0 0 48px",
          fontWeight: "400",
        }}
      >
        Not just Claude and Codex. Every agent you pay for — accessible from any device, across
        every machine you own.
      </motion.p>

      {/* CTAs */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.55, delay: 0.24 }}
        style={{
          display: "flex",
          gap: "12px",
          marginBottom: "80px",
          flexWrap: "wrap",
          justifyContent: "center",
        }}
      >
        <a
          href="#waitlist"
          style={{
            display: "inline-flex",
            alignItems: "center",
            padding: "12px 28px",
            fontSize: "15px",
            fontWeight: "600",
            color: "#09090B",
            background: "#FAFAFA",
            borderRadius: "10px",
            textDecoration: "none",
            letterSpacing: "-0.01em",
            transition: "opacity 0.15s",
          }}
          onMouseEnter={(e) => ((e.target as HTMLElement).style.opacity = "0.88")}
          onMouseLeave={(e) => ((e.target as HTMLElement).style.opacity = "1")}
        >
          Get Started
        </a>
        <a
          href="https://github.com/aryateja2106/lecoder-mconnect"
          target="_blank"
          rel="noopener noreferrer"
          style={{
            display: "inline-flex",
            alignItems: "center",
            gap: "8px",
            padding: "12px 28px",
            fontSize: "15px",
            fontWeight: "600",
            color: "#FAFAFA",
            background: "transparent",
            border: "1px solid #27272A",
            borderRadius: "10px",
            textDecoration: "none",
            letterSpacing: "-0.01em",
            transition: "border-color 0.15s",
          }}
          onMouseEnter={(e) => ((e.target as HTMLElement).style.borderColor = "#52525B")}
          onMouseLeave={(e) => ((e.target as HTMLElement).style.borderColor = "#27272A")}
        >
          View on GitHub
        </a>
      </motion.div>

      {/* Supported agents row */}
      <motion.div
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5, delay: 0.32 }}
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          gap: "14px",
          marginBottom: "64px",
        }}
      >
        <p
          style={{
            fontSize: "11px",
            fontWeight: "500",
            color: "#3F3F46",
            letterSpacing: "0.1em",
            textTransform: "uppercase",
            margin: "0",
          }}
        >
          Works with every agent you already use
        </p>
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "0",
            flexWrap: "wrap",
            justifyContent: "center",
          }}
        >
          {["Claude", "Cursor", "Amp", "Codex", "Droid", "Gemini", "Copilot"].map(
            (agent, i, arr) => (
              <span key={agent} style={{ display: "flex", alignItems: "center" }}>
                <span
                  style={{
                    fontSize: "13px",
                    fontWeight: "500",
                    color: "#71717A",
                    padding: "0 14px",
                    letterSpacing: "-0.01em",
                  }}
                >
                  {agent}
                </span>
                {i < arr.length - 1 && (
                  <span
                    style={{
                      width: "1px",
                      height: "14px",
                      background: "#27272A",
                      display: "inline-block",
                    }}
                  />
                )}
              </span>
            ),
          )}
          <span style={{ display: "flex", alignItems: "center" }}>
            <span
              style={{
                width: "1px",
                height: "14px",
                background: "#27272A",
                display: "inline-block",
              }}
            />
            <span
              style={{
                fontSize: "13px",
                fontWeight: "500",
                color: "#52525B",
                padding: "0 14px",
                letterSpacing: "-0.01em",
              }}
            >
              + any CLI agent
            </span>
          </span>
        </div>
      </motion.div>

      {/* Terminal Mockup */}
      {/* Outer clip wrapper: prevents terminal content from leaking into page-level scroll on mobile */}
      <div
        style={{
          width: "100%",
          maxWidth: "680px",
          overflow: "clip",
        }}
      >
      <motion.div
        initial={{ opacity: 0, y: 32 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.65, delay: 0.36 }}
        style={{
          width: "100%",
          borderRadius: "14px",
          border: "1px solid #27272A",
          background: "#0D0D0F",
          overflow: "hidden",
          boxShadow: "0 0 0 1px rgba(255,255,255,0.04) inset, 0 32px 64px rgba(0,0,0,0.6)",
        }}
      >
        {/* Terminal title bar */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            padding: "12px 16px",
            borderBottom: "1px solid #1C1C1F",
            gap: "8px",
          }}
        >
          <div style={{ display: "flex", gap: "6px" }}>
            {["#FF5F57", "#FEBC2E", "#28C840"].map((color) => (
              <div
                key={color}
                style={{
                  width: "12px",
                  height: "12px",
                  borderRadius: "50%",
                  background: color,
                  opacity: 0.7,
                }}
              />
            ))}
          </div>
          <span
            style={{
              flex: 1,
              textAlign: "center",
              fontSize: "12px",
              color: "#52525B",
              fontFamily: "'SF Mono', 'Fira Code', 'Fira Mono', 'Roboto Mono', monospace",
            }}
          >
            terminal — zsh
          </span>
        </div>

        {/* Terminal body */}
        <div
          style={{
            padding: "20px 24px",
            fontFamily: "'SF Mono', 'Fira Code', 'Fira Mono', 'Roboto Mono', monospace",
            fontSize: "13px",
            lineHeight: "1.7",
            overflowX: "auto",
          }}
        >
          {terminalLines.map((line, i) => {
            if (line.qr) {
              return (
                <div key={i} style={{ margin: "8px 0 8px 4px" }}>
                  <QRCode />
                </div>
              );
            }
            return (
              <div
                key={i}
                style={{
                  display: "flex",
                  alignItems: line.prompt ? "center" : "flex-start",
                  minHeight: line.text === "" ? "8px" : "auto",
                }}
              >
                {line.prompt && <span style={{ color: "#A1A1AA", marginRight: "8px" }}>~</span>}
                {line.prompt && <span style={{ color: "#52525B", marginRight: "8px" }}>$</span>}
                {line.prefix && (
                  <span
                    style={{
                      color: "#FAFAFA",
                      marginRight: "8px",
                      fontSize: "11px",
                    }}
                  >
                    {line.prefix}
                  </span>
                )}
                <span
                  style={{
                    color: line.prompt
                      ? "#FAFAFA"
                      : line.dim
                        ? "#52525B"
                        : line.code
                          ? "#e0e7ff"
                          : line.bold
                            ? "#FAFAFA"
                            : "#A1A1AA",
                    fontWeight: line.bold ? "600" : "400",
                  }}
                >
                  {line.text}
                  {line.cursor && (
                    <span
                      style={{
                        display: "inline-block",
                        width: "8px",
                        height: "14px",
                        background: "#FAFAFA",
                        marginLeft: "4px",
                        verticalAlign: "text-bottom",
                        animation: "blink 1.2s step-end infinite",
                      }}
                    />
                  )}
                </span>
              </div>
            );
          })}
        </div>
      </motion.div>
      </div>

      <style>{`
        @keyframes blink {
          0%, 100% { opacity: 1; }
          50% { opacity: 0; }
        }
      `}</style>
    </section>
  );
}
