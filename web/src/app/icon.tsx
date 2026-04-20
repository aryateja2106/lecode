import { ImageResponse } from "next/og";

export const size = { width: 32, height: 32 };
export const contentType = "image/png";

export default function Icon() {
  return new ImageResponse(
    <div
      style={{
        fontSize: 22,
        background: "#09090B",
        width: "100%",
        height: "100%",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        color: "#FAFAFA",
        borderRadius: 6,
        fontWeight: 700,
        fontFamily: "-apple-system, BlinkMacSystemFont, sans-serif",
      }}
    >
      L
    </div>,
    { ...size },
  );
}
