import { test, expect } from "@playwright/test";

test.describe("Hero terminal mobile overflow", () => {
  test("no horizontal body scroll at iPhone 13 viewport (390x844)", async ({
    page,
  }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto("/");

    // Wait for the page to fully render (animations settle)
    await page.waitForTimeout(800);

    const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
    const innerWidth = await page.evaluate(() => window.innerWidth);

    expect(scrollWidth).toBeLessThanOrEqual(innerWidth);
  });

  test("terminal container is visible at iPhone 13 viewport", async ({
    page,
  }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto("/");

    // Wait for animations
    await page.waitForTimeout(800);

    // The terminal is identified by the macOS traffic-light buttons (red/yellow/green dots)
    // and the "terminal — zsh" title. Select by the title text.
    const terminalTitle = page.getByText("terminal — zsh");
    await expect(terminalTitle).toBeVisible();
  });
});
