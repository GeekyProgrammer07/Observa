import { render, screen } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ProtectedRoute } from "./ProtectedRoute";
import { useAuth } from "@/lib/auth";

vi.mock("@/lib/auth", () => ({
  useAuth: vi.fn(),
}));

function renderProtected() {
  return render(
    <MemoryRouter initialEntries={["/dashboard"]}>
      <Routes>
        <Route
          path="/dashboard"
          element={
            <ProtectedRoute>
              <p>dashboard content</p>
            </ProtectedRoute>
          }
        />
        <Route path="/login" element={<p>login page</p>} />
      </Routes>
    </MemoryRouter>
  );
}

function mockAuth(state: { isAuthenticated: boolean; isLoading: boolean }) {
  vi.mocked(useAuth).mockReturnValue({
    token: state.isAuthenticated ? "tok" : null,
    isAuthenticated: state.isAuthenticated,
    isLoading: state.isLoading,
    signin: vi.fn(),
    signup: vi.fn(),
    signout: vi.fn(),
  });
}

describe("ProtectedRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("shows a loading spinner while auth state is loading", () => {
    mockAuth({ isAuthenticated: false, isLoading: true });
    const { container } = renderProtected();

    expect(container.querySelector(".animate-spin")).toBeTruthy();
    expect(screen.queryByText("dashboard content")).toBeNull();
    expect(screen.queryByText("login page")).toBeNull();
  });

  it("redirects unauthenticated users to /login", () => {
    mockAuth({ isAuthenticated: false, isLoading: false });
    renderProtected();

    expect(screen.getByText("login page")).toBeTruthy();
    expect(screen.queryByText("dashboard content")).toBeNull();
  });

  it("renders children for authenticated users", () => {
    mockAuth({ isAuthenticated: true, isLoading: false });
    renderProtected();

    expect(screen.getByText("dashboard content")).toBeTruthy();
    expect(screen.queryByText("login page")).toBeNull();
  });
});
