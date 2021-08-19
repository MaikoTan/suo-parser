import { expect } from "chai";
import { generateAsync, parseAsync, transformAsync } from "../src";

describe("Generator", () => {
  it("should generate simple timeline entry", async () => {
    const timelineText = '0.0 "--Reset--" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0';
    const ast = await parseAsync(timelineText);
    const timeline = await generateAsync(ast);
    expect(timeline).to.equal(timelineText);
  });

  it("should generate hideall statement", async () => {
    const timelineText = 'hideall "--sync--"';
    const ast = await parseAsync(timelineText);
    const timeline = await generateAsync(ast);
    expect(timeline).to.equal(timelineText);
  });

  it("should transform timeline", async () => {
    const timelineText = '0.0 "--Reset--" sync / 00:0839:.*is no longer sealed/ duration 5 window 10000 jump 0';
    const timeline = await transformAsync(timelineText);
    expect(timeline).to.equal(timelineText);
  });

  it("should transform hideall statement", async () => {
    const timelineText = 'hideall "--sync--"';
    const timeline = await transformAsync(timelineText);
    expect(timeline).to.equal(timelineText);
  });
});
