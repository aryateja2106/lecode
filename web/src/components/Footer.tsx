"use client";

export default function Footer() {
  return (
    <footer
      style={{
        borderTop: "1px solid #18181B",
        padding: "48px 24px",
      }}
    >
      <div
        style={{
          maxWidth: "1120px",
          margin: "0 auto",
          display: "flex",
          flexDirection: "row",
          alignItems: "center",
          justifyContent: "space-between",
          flexWrap: "wrap",
          gap: "24px",
        }}
      >
        {/* Left: Logo + tagline */}
        <div>
          <div
            style={{
              fontSize: "16px",
              fontWeight: "700",
              color: "#FAFAFA",
              letterSpacing: "-0.02em",
              marginBottom: "6px",
            }}
          >
            LeSearch AI
          </div>
          <div
            style={{
              fontSize: "13px",
              color: "#3F3F46",
            }}
          >
            Built in San Francisco
          </div>
        </div>

        {/* Center: Links */}
        <nav
          style={{
            display: "flex",
            gap: "28px",
            alignItems: "center",
          }}
        >
          {[
            {
              label: "GitHub",
              href: "https://github.com/aryateja2106/lecoder-mconnect",
            },
            { label: "X", href: "https://x.com/aryateja_r" },
            { label: "Docs", href: "#" },
          ].map((link) => (
            <a
              key={link.label}
              href={link.href}
              target={link.href.startsWith("http") ? "_blank" : undefined}
              rel={link.href.startsWith("http") ? "noopener noreferrer" : undefined}
              style={{
                fontSize: "13px",
                color: "#52525B",
                textDecoration: "none",
                transition: "color 0.15s",
              }}
              onMouseEnter={(e) => ((e.target as HTMLElement).style.color = "#A1A1AA")}
              onMouseLeave={(e) => ((e.target as HTMLElement).style.color = "#52525B")}
            >
              {link.label}
            </a>
          ))}
        </nav>

        {/* Right: Copyright */}
        <div
          style={{
            fontSize: "13px",
            color: "#3F3F46",
          }}
        >
          2026 LeSearch AI
        </div>
      </div>
    </footer>
  );
}
