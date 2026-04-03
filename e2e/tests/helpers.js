const GAME_URL = process.env.BASE_URL || 'https://prdpx7.github.io/Cheerio/';

async function waitForGame(page, timeoutMs = 30000) {
    await page.goto(GAME_URL, { timeout: timeoutMs });
    await page.waitForTimeout(10000);
}

async function goToGame(page, timeoutMs = 30000) {
    await page.goto(GAME_URL, { timeout: timeoutMs });
}

async function getCanvasBox(page) {
    return page.locator('#glcanvas').boundingBox();
}

async function tapCanvas(page, xRatio = 0.5, yRatio = 0.5) {
    const box = await getCanvasBox(page);
    await page.tap('#glcanvas', {
        position: { x: box.width * xRatio, y: box.height * yRatio }
    });
}

async function startGame(page, isMobile) {
    if (isMobile) {
        await tapCanvas(page, 0.5, 0.5);
    } else {
        await page.keyboard.press('Space');
    }
    await page.waitForTimeout(500);
}

async function jump(page, isMobile) {
    if (isMobile) {
        await tapCanvas(page, 0.3, 0.5);
    } else {
        await page.keyboard.press('Space');
    }
}

module.exports = { GAME_URL, waitForGame, goToGame, getCanvasBox, tapCanvas, startGame, jump };
