import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Button } from "./Button";

describe("Button", () => {
  it("renders its children", () => {
    render(<Button>Save changes</Button>);
    expect(screen.getByRole("button", { name: "Save changes" })).toBeTruthy();
  });

  it("fires onClick when clicked", () => {
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Go</Button>);

    fireEvent.click(screen.getByRole("button"));
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it("does not fire onClick when disabled", () => {
    const onClick = vi.fn();
    render(
      <Button onClick={onClick} disabled>
        Go
      </Button>
    );

    fireEvent.click(screen.getByRole("button"));
    expect(onClick).not.toHaveBeenCalled();
  });

  it("is disabled and shows a spinner while loading", () => {
    const { container } = render(<Button loading>Submitting</Button>);

    const button = screen.getByRole("button") as HTMLButtonElement;
    expect(button.disabled).toBe(true);
    expect(container.querySelector("svg.animate-spin")).toBeTruthy();
  });

  it("shows no spinner when not loading", () => {
    const { container } = render(<Button>Idle</Button>);
    expect(container.querySelector("svg.animate-spin")).toBeNull();
  });

  it.each([
    ["primary", "bg-indigo-600"],
    ["secondary", "bg-slate-700"],
    ["danger", "bg-red-600"],
    ["ghost", "bg-transparent"],
  ] as const)("applies the %s variant classes", (variant, expectedClass) => {
    render(<Button variant={variant}>V</Button>);
    expect(screen.getByRole("button").className).toContain(expectedClass);
  });

  it("defaults to the primary variant and md size", () => {
    render(<Button>Default</Button>);
    const cls = screen.getByRole("button").className;
    expect(cls).toContain("bg-indigo-600");
    expect(cls).toContain("px-4 py-2");
  });

  it("merges custom className with its own classes", () => {
    render(<Button className="w-full">Wide</Button>);
    const cls = screen.getByRole("button").className;
    expect(cls).toContain("w-full");
    expect(cls).toContain("rounded-lg");
  });

  it("forwards native button props like type", () => {
    render(<Button type="submit">Submit</Button>);
    expect((screen.getByRole("button") as HTMLButtonElement).type).toBe("submit");
  });
});
