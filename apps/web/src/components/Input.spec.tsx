import { createRef } from "react";
import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Input } from "./Input";

describe("Input", () => {
  it("associates the label with the input via a derived id", () => {
    render(<Input label="Monitor URL" />);

    const input = screen.getByLabelText("Monitor URL") as HTMLInputElement;
    expect(input.id).toBe("monitor-url");
  });

  it("prefers an explicit id over the derived one", () => {
    render(<Input label="Monitor URL" id="custom-id" />);

    const input = screen.getByLabelText("Monitor URL") as HTMLInputElement;
    expect(input.id).toBe("custom-id");
  });

  it("renders without a label", () => {
    render(<Input placeholder="type here" />);
    expect(screen.getByPlaceholderText("type here")).toBeTruthy();
    expect(document.querySelector("label")).toBeNull();
  });

  it("shows the error message and applies the error border", () => {
    render(<Input label="Username" error="Username is required" />);

    expect(screen.getByText("Username is required")).toBeTruthy();
    expect(screen.getByLabelText("Username").className).toContain("border-red-500");
  });

  it("shows no error styling when there is no error", () => {
    render(<Input label="Username" />);

    expect(screen.queryByText("Username is required")).toBeNull();
    expect(screen.getByLabelText("Username").className).not.toContain("border-red-500");
  });

  it("propagates change events", () => {
    const onChange = vi.fn();
    render(<Input label="Name" onChange={onChange} />);

    fireEvent.change(screen.getByLabelText("Name"), { target: { value: "obs" } });
    expect(onChange).toHaveBeenCalledTimes(1);
  });

  it("forwards its ref to the underlying input element", () => {
    const ref = createRef<HTMLInputElement>();
    render(<Input label="Focus me" ref={ref} />);

    expect(ref.current).toBeInstanceOf(HTMLInputElement);
    expect(ref.current?.id).toBe("focus-me");
  });

  it("passes through native input props", () => {
    render(<Input label="Password" type="password" required />);

    const input = screen.getByLabelText("Password") as HTMLInputElement;
    expect(input.type).toBe("password");
    expect(input.required).toBe(true);
  });
});
