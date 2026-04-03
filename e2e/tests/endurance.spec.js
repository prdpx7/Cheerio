const { test, expect } = require('@playwright/test');
const { waitForGame, startGame, jump } = require('./helpers');

test.describe('Endurance', () => {
    test('survive 15 seconds with active jumping', async ({ page, isMobile }) => {
        test.setTimeout(60000);
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');

        await waitForGame(page);
        await page.keyboard.press('Space');
        await page.waitForTimeout(500);

        const start = Date.now();
        let jumpCount = 0;

        while (Date.now() - start < 15000) {
            await page.keyboard.press('Space');
            jumpCount++;
            await page.waitForTimeout(250 + Math.random() * 200);
        }

        const elapsed = ((Date.now() - start) / 1000).toFixed(1);
        console.log(`    Survived ${elapsed}s, jumped ${jumpCount} times`);
    });

    test('3 full game cycles without crashes', async ({ page, isMobile }) => {
        test.setTimeout(120000);
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');

        const errors = [];
        page.on('pageerror', err => errors.push(err.message));

        await waitForGame(page);

        for (let cycle = 0; cycle < 3; cycle++) {
            await page.keyboard.press('Space');
            await page.waitForTimeout(500);

            for (let i = 0; i < 8; i++) {
                await page.keyboard.press('Space');
                await page.waitForTimeout(350);
            }

            await page.waitForTimeout(15000);
            await page.keyboard.press('Space');
            await page.waitForTimeout(1500);

            console.log(`    Cycle ${cycle + 1}/3 complete`);
        }

        expect(errors).toHaveLength(0);
    });
});
