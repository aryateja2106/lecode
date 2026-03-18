"use client";

import FadeIn from "@/components/FadeIn";

export default function Architecture() {
  return (
    <section
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
          Architecture
        </p>
        <h2
          style={{
            fontSize: "clamp(32px, 4.5vw, 52px)",
            fontWeight: "800",
            letterSpacing: "-0.035em",
            lineHeight: "1.08",
            color: "#FAFAFA",
            margin: "0 0 20px",
            maxWidth: "520px",
          }}
        >
          How the routing works
        </h2>
        <p
          style={{
            fontSize: "17px",
            lineHeight: "1.65",
            color: "#71717A",
            maxWidth: "480px",
            margin: "0 0 72px",
          }}
        >
          Your devices connect through LeSearch Cloud routing, which maintains
          secure tunnels to all your machines. No direct exposure, no open
          ports.
        </p>
      </FadeIn>

      <FadeIn delay={120}>
        <div
          style={{
            border: "1px solid #27272A",
            borderRadius: "16px",
            background: "#0D0D0F",
            padding: "48px 40px",
            overflowX: "auto",
          }}
        >
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              minWidth: "500px",
            }}
          >
            {/* Your devices row */}
            <div
              style={{ display: "flex", alignItems: "center", gap: "12px" }}
            >
              {[
                ["Phone", "MConnect iOS"],
                ["Laptop", "Browser / SSH"],
                ["Tablet", "Any device"],
              ].map(([label, sub]) => (
                <div
                  key={label}
                  style={{
                    padding: "14px 20px",
                    border: "1px solid #27272A",
                    borderRadius: "10px",
                    background: "#0D0D0F",
                    textAlign: "center",
                    minWidth: "110px",
                  }}
                >
                  <div
                    style={{
                      fontSize: "13px",
                      fontWeight: "600",
                      color: "#FAFAFA",
                    }}
                  >
                    {label}
                  </div>
                  <div
                    style={{ fontSize: "11px", color: "#52525B", marginTop: "4px" }}
                  >
                    {sub}
                  </div>
                </div>
              ))}
            </div>

            {/* Arrow */}
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                padding: "8px 0",
              }}
            >
              <div style={{ width: "1px", height: "40px", background: "#27272A" }} />
              <div
                style={{
                  width: 0,
                  height: 0,
                  borderLeft: "5px solid transparent",
                  borderRight: "5px solid transparent",
                  borderTop: "6px solid #3F3F46",
                }}
              />
            </div>

            {/* Cloud routing */}
            <div
              style={{
                padding: "20px 40px",
                border: "1px solid #3F3F46",
                borderRadius: "12px",
                background: "#18181B",
                textAlign: "center",
              }}
            >
              <div
                style={{
                  fontSize: "14px",
                  fontWeight: "700",
                  color: "#FAFAFA",
                  marginBottom: "12px",
                }}
              >
                LeSearch Cloud
              </div>
              <div
                style={{
                  display: "flex",
                  gap: "10px",
                  flexWrap: "wrap",
                  justifyContent: "center",
                }}
              >
                {["Headscale VPN", "Cloudflare Tunnel", "SSH Proxy", "VNC Gateway"].map(
                  (label) => (
                    <span
                      key={label}
                      style={{
                        padding: "3px 10px",
                        fontSize: "11px",
                        color: "#71717A",
                        border: "1px solid #27272A",
                        borderRadius: "4px",
                        fontFamily:
                          "'SF Mono', 'Fira Code', 'Fira Mono', 'Roboto Mono', monospace",
                      }}
                    >
                      {label}
                    </span>
                  )
                )}
              </div>
            </div>

            {/* Arrow */}
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                padding: "8px 0",
              }}
            >
              <div style={{ width: "1px", height: "40px", background: "#27272A" }} />
              <div
                style={{
                  width: 0,
                  height: 0,
                  borderLeft: "5px solid transparent",
                  borderRight: "5px solid transparent",
                  borderTop: "6px solid #3F3F46",
                }}
              />
            </div>

            {/* Your machines */}
            <div
              style={{ display: "flex", alignItems: "center", gap: "12px" }}
            >
              {[
                ["Raspberry Pi", "Dallas, TX · 24/7"],
                ["Server", "Cloud / bare metal"],
                ["Workstation", "Home / office"],
              ].map(([label, sub]) => (
                <div
                  key={label}
                  style={{
                    padding: "14px 20px",
                    border: "1px solid #27272A",
                    borderRadius: "10px",
                    background: "#0D0D0F",
                    textAlign: "center",
                    minWidth: "110px",
                  }}
                >
                  <div
                    style={{
                      fontSize: "13px",
                      fontWeight: "600",
                      color: "#FAFAFA",
                    }}
                  >
                    {label}
                  </div>
                  <div
                    style={{ fontSize: "11px", color: "#52525B", marginTop: "4px" }}
                  >
                    {sub}
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Legend */}
          <div
            style={{
              marginTop: "40px",
              paddingTop: "24px",
              borderTop: "1px solid #1C1C1F",
              display: "flex",
              gap: "24px",
              flexWrap: "wrap",
              justifyContent: "center",
            }}
          >
            {[
              ["No open ports", "Cloudflare handles ingress"],
              ["E2E encrypted", "TLS everywhere"],
              ["Zero config", "QR code pairing"],
              ["Agent-aware", "Built for AI workflows"],
            ].map(([title, desc]) => (
              <div key={title} style={{ textAlign: "center" }}>
                <div
                  style={{
                    fontSize: "13px",
                    fontWeight: "600",
                    color: "#FAFAFA",
                    marginBottom: "2px",
                  }}
                >
                  {title}
                </div>
                <div style={{ fontSize: "12px", color: "#52525B" }}>{desc}</div>
              </div>
            ))}
          </div>
        </div>
      </FadeIn>
    </section>
  );
}
