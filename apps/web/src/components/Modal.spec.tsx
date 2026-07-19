import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Modal } from "./Modal";

describe("Modal", () => {
  it("renders nothing when closed", () => {
    const { container } = render(
      <Modal open={false} title="Hidden" onClose={vi.fn()}>
        <p>secret</p>
      </Modal>
    );

    expect(container.innerHTML).toBe("");
    expect(screen.queryByText("secret")).toBeNull();
  });

  it("renders the title and children when open", () => {
    render(
      <Modal open title="Create monitor" onClose={vi.fn()}>
        <p>modal body</p>
      </Modal>
    );

    expect(screen.getByRole("heading", { name: "Create monitor" })).toBeTruthy();
    expect(screen.getByText("modal body")).toBeTruthy();
  });

  it("calls onClose when the close button is clicked", () => {
    const onClose = vi.fn();
    render(
      <Modal open title="T" onClose={onClose}>
        <p>body</p>
      </Modal>
    );

    fireEvent.click(screen.getByRole("button"));
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("calls onClose when the backdrop is clicked", () => {
    const onClose = vi.fn();
    const { container } = render(
      <Modal open title="T" onClose={onClose}>
        <p>body</p>
      </Modal>
    );

    const backdrop = container.querySelector(".bg-black\\/60");
    expect(backdrop).toBeTruthy();
    fireEvent.click(backdrop as Element);
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("does not close when clicking inside the dialog content", () => {
    const onClose = vi.fn();
    render(
      <Modal open title="T" onClose={onClose}>
        <p>body</p>
      </Modal>
    );

    fireEvent.click(screen.getByText("body"));
    expect(onClose).not.toHaveBeenCalled();
  });

  it("calls onClose when Escape is pressed while open", () => {
    const onClose = vi.fn();
    render(
      <Modal open title="T" onClose={onClose}>
        <p>body</p>
      </Modal>
    );

    fireEvent.keyDown(document, { key: "Escape" });
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("ignores other keys", () => {
    const onClose = vi.fn();
    render(
      <Modal open title="T" onClose={onClose}>
        <p>body</p>
      </Modal>
    );

    fireEvent.keyDown(document, { key: "Enter" });
    expect(onClose).not.toHaveBeenCalled();
  });

  it("removes the Escape listener when closed", () => {
    const onClose = vi.fn();
    const { rerender } = render(
      <Modal open title="T" onClose={onClose}>
        <p>body</p>
      </Modal>
    );

    rerender(
      <Modal open={false} title="T" onClose={onClose}>
        <p>body</p>
      </Modal>
    );

    fireEvent.keyDown(document, { key: "Escape" });
    expect(onClose).not.toHaveBeenCalled();
  });
});
