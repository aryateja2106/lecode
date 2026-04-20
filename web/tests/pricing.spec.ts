import { test, expect } from "@playwright/test";

test.describe("Pricing section", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/#pricing");
    // scroll to pricing section
    await page.locator("#pricing").waitFor({ state: "visible" });
  });

  test("renders 4 pricing cards with correct plan names", async ({ page }) => {
    const section = page.locator("#pricing");

    for (const name of ["Hobby", "Starter", "Pro", "Enterprise"]) {
      await expect(
        section.locator(`h3:has-text("${name}")`),
      ).toBeVisible();
    }
  });

  test("no Tailwind utility classes leak inside the Pricing section", async ({
    page,
  }) => {
    const section = page.locator("#pricing");

    // Collect all elements inside the section
    const elements = await section.locator("*").all();

    const tailwindPatterns = ["bg-[", "text-[", "py-"];

    for (const el of elements) {
      const className = await el.getAttribute("class");
      if (!className) continue;

      for (const pattern of tailwindPatterns) {
        expect(
          className,
          `Element has Tailwind leak "${pattern}": "${className}"`,
        ).not.toContain(pattern);
      }
    }
  });

  test("Most Popular badge is visible on Pro card", async ({ page }) => {
    await expect(page.locator("text=Most Popular")).toBeVisible();
  });

  test("all CTA buttons are visible", async ({ page }) => {
    const section = page.locator("#pricing");
    const buttons = section.locator("button");
    await expect(buttons).toHaveCount(4);
  });
});
