"use client";

import { useState } from "react";
import FadeIn from "@/components/FadeIn";

interface Plan {
  name: string;
  price: string;
  period?: string;
  features: string[];
  cta: string;
  popular?: boolean;
}

const plans: Plan[] = [
  {
    name: "Hobby",
    price: "$0",
    features: [
      "5 sessions/month",
      "$10 cloud sandbox credit",
      "Web and mobile access",
      "Community support",
    ],
    cta: "Start Free",
  },
  {
    name: "Starter",
    price: "$9",
    period: "/month",
    features: [
      "25 sessions/month",
      "$40 cloud sandbox credit",
      "Web and mobile access",
      "Email support",
    ],
    cta: "Get Started",
  },
  {
    name: "Pro",
    price: "$20",
    period: "/month",
    features: [
      "Unlimited sessions",
      "$100 cloud sandbox credit",
      "Priority support",
      "Locked-in price",
      "All future features",
    ],
    cta: "Get Started",
    popular: true,
  },
  {
    name: "Enterprise",
    price: "Custom",
    features: [
      "Unlimited everything",
      "Team collaboration",
      "Dedicated support",
      "Custom integrations",
      "99.9% uptime SLA",
    ],
    cta: "Schedule a Call",
  },
];

function CheckIcon({ accent }: { accent?: boolean }) {
  return (
    <svg
      width="16"
      height="16"
      viewBox="0 0 16 16"
      fill="none"
      style={{ flexShrink: 0, marginTop: "2px" }}
    >
      <circle cx="8" cy="8" r="7.5" stroke={accent ? "#FAFAFA" : "#3F3F46"} />
      <path
        d="M4.5 8L7 10.5L11.5 6"
        stroke={accent ? "#09090B" : "#FAFAFA"}
        strokeWidth="1.5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

function PlanCard({ plan, index }: { plan: Plan; index: number }) {
  const [hovered, setHovered] = useState(false);

  const isPopular = plan.popular === true;

  const cardStyle: React.CSSProperties = {
    position: "relative",
    border: isPopular ? "1px solid #52525B" : "1px solid #27272A",
    borderRadius: "16px",
    padding: "32px",
    background: isPopular ? "#18181B" : "#0D0D0F",
    flex: "1 1 220px",
    minWidth: "200px",
    boxShadow: isPopular ? "0 8px 32px rgba(0,0,0,0.5)" : "none",
    display: "flex",
    flexDirection: "column",
  };

  const buttonStyle: React.CSSProperties = {
    width: "100%",
    padding: "12px 24px",
    borderRadius: "10px",
    fontSize: "15px",
    fontWeight: "500",
    cursor: "pointer",
    border: "none",
    transition: "background 0.15s ease",
    background: isPopular
      ? hovered
        ? "#E4E4E7"
        : "#FAFAFA"
      : hovered
        ? "#27272A"
        : "#18181B",
    color: isPopular ? "#09090B" : "#FAFAFA",
    marginTop: "auto",
  };

  return (
    <div style={cardStyle}>
      {isPopular && (
        <div
          style={{
            position: "absolute",
            top: "-14px",
            left: "50%",
            transform: "translateX(-50%)",
            background: "#FAFAFA",
            color: "#09090B",
            padding: "4px 16px",
            borderRadius: "9999px",
            fontSize: "12px",
            fontWeight: "600",
            whiteSpace: "nowrap",
          }}
        >
          Most Popular
        </div>
      )}

      <h3
        style={{
          fontSize: "22px",
          fontWeight: "700",
          color: "#FAFAFA",
          margin: "0 0 8px",
        }}
      >
        {plan.name}
      </h3>

      <div style={{ marginBottom: "24px" }}>
        <span
          style={{
            fontSize: "36px",
            fontWeight: "800",
            color: "#FAFAFA",
            letterSpacing: "-0.02em",
          }}
        >
          {plan.price}
        </span>
        {plan.period && (
          <span style={{ fontSize: "15px", color: "#71717A", marginLeft: "2px" }}>
            {plan.period}
          </span>
        )}
      </div>

      <ul
        style={{
          listStyle: "none",
          margin: "0 0 28px",
          padding: 0,
          display: "flex",
          flexDirection: "column",
          gap: "14px",
          flexGrow: 1,
        }}
      >
        {plan.features.map((feature) => (
          <li
            key={feature}
            style={{
              display: "flex",
              alignItems: "flex-start",
              gap: "10px",
            }}
          >
            <CheckIcon accent={isPopular} />
            <span
              style={{
                fontSize: "14px",
                color: isPopular ? "#D4D4D8" : "#A1A1AA",
                lineHeight: "1.5",
              }}
            >
              {feature}
            </span>
          </li>
        ))}
      </ul>

      <button
        style={buttonStyle}
        onMouseEnter={() => setHovered(true)}
        onMouseLeave={() => setHovered(false)}
      >
        {plan.cta}
      </button>
    </div>
  );
}

export default function Pricing() {
  return (
    <section
      id="pricing"
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
          Pricing
        </p>
        <h2
          style={{
            fontSize: "clamp(32px, 4.5vw, 52px)",
            fontWeight: "800",
            letterSpacing: "-0.035em",
            lineHeight: "1.08",
            color: "#FAFAFA",
            margin: "0 0 20px",
            maxWidth: "560px",
          }}
        >
          Simple, transparent pricing
        </h2>
        <p
          style={{
            fontSize: "17px",
            lineHeight: "1.65",
            color: "#71717A",
            maxWidth: "480px",
            margin: "0 0 56px",
          }}
        >
          Choose the plan that fits your needs. Start free, upgrade when you
          are ready.
        </p>
      </FadeIn>

      <FadeIn delay={80}>
        <div
          style={{
            display: "flex",
            flexWrap: "wrap",
            gap: "16px",
            alignItems: "stretch",
          }}
        >
          {plans.map((plan, i) => (
            <PlanCard key={plan.name} plan={plan} index={i} />
          ))}
        </div>
      </FadeIn>
    </section>
  );
}
