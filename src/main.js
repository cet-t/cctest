import { invoke } from "@tauri-apps/api/core";

// State
const state = {
  isListening: false,
  currentTheme: "sync",
  gesturePoints: [],
  isDrawing: false,
};

// Initialize on load
document.addEventListener("DOMContentLoaded", () => {
  initializeApp();
  loadSettings();
  loadTheme();
  setupEventListeners();
  updateDashboard();
});

function initializeApp() {
  setupNavigation();
  setupGestureRecorder();
  setupSettingsForms();
  setupThemeSettings();
}

// Navigation
function setupNavigation() {
  const navButtons = document.querySelectorAll(".nav-button");
  navButtons.forEach((button) => {
    button.addEventListener("click", (e) => {
      const pageName = e.target.dataset.page;
      navigateTo(pageName);
    });
  });
}

function navigateTo(pageName) {
  // Hide all pages
  const pages = document.querySelectorAll(".page");
  pages.forEach((page) => page.classList.remove("active"));

  // Show selected page
  const selectedPage = document.getElementById(pageName);
  if (selectedPage) {
    selectedPage.classList.add("active");
  }

  // Update nav button active state
  const navButtons = document.querySelectorAll(".nav-button");
  navButtons.forEach((btn) => btn.classList.remove("active"));
  document.querySelector(`[data-page="${pageName}"]`).classList.add("active");
}

// Dashboard
async function updateDashboard() {
  try {
    const settings = await invoke("get_global_settings");
    document.getElementById("status-enabled").textContent = settings.enabled
      ? "Yes"
      : "No";

    const activeWindow = await invoke("get_active_window");
    document.getElementById("status-window").textContent = activeWindow;
  } catch (error) {
    console.error("Error updating dashboard:", error);
  }
}

// Gesture Recorder
function setupGestureRecorder() {
  const canvas = document.getElementById("gesture-canvas");
  const ctx = canvas.getContext("2d");

  // Set canvas size
  canvas.width = canvas.offsetWidth;
  canvas.height = canvas.offsetHeight;

  // Mouse events for gesture drawing
  canvas.addEventListener("mousedown", (e) => {
    if (e.button === 2) {
      // Right mouse button
      state.isDrawing = true;
      state.gesturePoints = [];
      const rect = canvas.getBoundingClientRect();
      const point = {
        x: Math.round(e.clientX - rect.left),
        y: Math.round(e.clientY - rect.top),
        timestamp: Date.now(),
      };
      state.gesturePoints.push(point);
    }
  });

  canvas.addEventListener("mousemove", (e) => {
    if (state.isDrawing) {
      const rect = canvas.getBoundingClientRect();
      const point = {
        x: Math.round(e.clientX - rect.left),
        y: Math.round(e.clientY - rect.top),
        timestamp: Date.now(),
      };
      state.gesturePoints.push(point);

      // Draw gesture
      drawGesture(ctx, state.gesturePoints);
    }
  });

  canvas.addEventListener("mouseup", () => {
    state.isDrawing = false;
  });

  canvas.addEventListener("contextmenu", (e) => {
    e.preventDefault();
  });

  // Clear button
  document.getElementById("clear-gesture").addEventListener("click", () => {
    state.gesturePoints = [];
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    document.getElementById("gesture-result").classList.remove("show");
  });

  // Save button
  document.getElementById("save-gesture").addEventListener("click", async () => {
    const gestureName = document.getElementById("gesture-name").value;
    if (!gestureName) {
      alert("Please enter a gesture name");
      return;
    }

    if (state.gesturePoints.length < 3) {
      alert("Please draw a gesture first");
      return;
    }

    try {
      const result = await invoke("detect_gesture", {
        points: state.gesturePoints,
      });
      console.log("Gesture detected:", result);

      const resultDiv = document.getElementById("gesture-result");
      resultDiv.innerHTML = `
        <strong>Gesture Detected:</strong> ${result.gesture}<br>
        <strong>Confidence:</strong> ${(result.confidence * 100).toFixed(0)}%<br>
        <strong>Name:</strong> ${gestureName}
      `;
      resultDiv.classList.add("show");

      // Clear for next gesture
      setTimeout(() => {
        state.gesturePoints = [];
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        document.getElementById("gesture-name").value = "";
      }, 2000);
    } catch (error) {
      alert("Error detecting gesture: " + error);
    }
  });
}

function drawGesture(ctx, points) {
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);

  if (points.length < 2) return;

  ctx.strokeStyle = "#007bff";
  ctx.lineWidth = 2;
  ctx.lineCap = "round";
  ctx.lineJoin = "round";

  ctx.beginPath();
  ctx.moveTo(points[0].x, points[0].y);

  for (let i = 1; i < points.length; i++) {
    ctx.lineTo(points[i].x, points[i].y);
  }

  ctx.stroke();

  // Draw start point
  ctx.fillStyle = "#28a745";
  ctx.beginPath();
  ctx.arc(points[0].x, points[0].y, 5, 0, 2 * Math.PI);
  ctx.fill();

  // Draw end point
  ctx.fillStyle = "#dc3545";
  ctx.beginPath();
  ctx.arc(points[points.length - 1].x, points[points.length - 1].y, 5, 0, 2 * Math.PI);
  ctx.fill();
}

// Settings
function setupSettingsForms() {
  // Settings tab switching
  document.querySelectorAll(".settings-tab-button").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      const tabName = e.target.dataset.tab;
      switchSettingsTab(tabName);
    });
  });

  // Global settings
  document
    .getElementById("save-global-settings")
    .addEventListener("click", saveGlobalSettings);

  // Sensitivity slider
  document
    .getElementById("global-sensitivity")
    .addEventListener("input", (e) => {
      document.getElementById("sensitivity-value").textContent = e.target.value;
    });

  // Per-app settings
  document
    .getElementById("save-app-settings")
    .addEventListener("click", saveAppSettings);
}

function switchSettingsTab(tabName) {
  // Hide all tabs
  document.querySelectorAll(".settings-tab").forEach((tab) => {
    tab.classList.remove("active");
  });

  // Show selected tab
  document.getElementById(`${tabName}-tab`).classList.add("active");

  // Update button states
  document.querySelectorAll(".settings-tab-button").forEach((btn) => {
    btn.classList.remove("active");
  });
  document.querySelector(`[data-tab="${tabName}"]`).classList.add("active");
}

async function loadSettings() {
  try {
    const settings = await invoke("get_global_settings");
    document.getElementById("global-enabled").checked = settings.enabled;
    document.getElementById("global-sensitivity").value = settings.sensitivity;
    document.getElementById("sensitivity-value").textContent =
      settings.sensitivity;
    document.getElementById("global-min-length").value =
      settings.min_gesture_length;
  } catch (error) {
    console.error("Error loading settings:", error);
  }
}

async function saveGlobalSettings() {
  const config = {
    enabled: document.getElementById("global-enabled").checked,
    sensitivity: parseFloat(
      document.getElementById("global-sensitivity").value
    ),
    min_gesture_length: parseInt(
      document.getElementById("global-min-length").value
    ),
  };

  try {
    await invoke("update_global_settings", { config });
    alert("Global settings saved!");
  } catch (error) {
    alert("Error saving settings: " + error);
  }
}

async function saveAppSettings() {
  const appName = document.getElementById("app-selector").value;
  if (!appName) {
    alert("Please select or enter an application name");
    return;
  }

  const config = {
    appName: appName,
    overrides: {
      enabled: true,
      disabledGestures: [],
      customGestures: {},
    },
  };

  try {
    await invoke("update_app_settings", { app_name: appName, config });
    alert("App settings saved!");
  } catch (error) {
    alert("Error saving app settings: " + error);
  }
}

// Theme Settings
function setupThemeSettings() {
  document.querySelectorAll('input[name="theme-mode"]').forEach((radio) => {
    radio.addEventListener("change", (e) => {
      const mode = e.target.value;
      applyTheme(mode);
    });
  });

  document.getElementById("save-theme").addEventListener("click", saveTheme);
}

function loadTheme() {
  // Default to sync theme
  applyTheme("sync");
}

function applyTheme(mode) {
  const customSection = document.getElementById("custom-theme-section");

  // Prevent redraw by using CSS class instead of style manipulation
  const root = document.documentElement;

  if (mode === "sync") {
    const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    root.setAttribute("data-theme", isDark ? "dark" : "light");
    customSection.style.display = "none";
  } else if (mode === "light") {
    root.removeAttribute("data-theme");
    customSection.style.display = "none";
  } else if (mode === "dark") {
    root.setAttribute("data-theme", "dark");
    customSection.style.display = "none";
  } else if (mode === "custom") {
    customSection.style.display = "block";
    applyCustomTheme();
  }

  state.currentTheme = mode;
  document.querySelector(`input[value="${mode}"]`).checked = true;
}

function applyCustomTheme() {
  const root = document.documentElement;
  const primary = document.getElementById("theme-primary").value;
  const secondary = document.getElementById("theme-secondary").value;
  const background = document.getElementById("theme-background").value;
  const text = document.getElementById("theme-text").value;

  root.style.setProperty("--color-primary", primary);
  root.style.setProperty("--color-secondary", secondary);
  root.style.setProperty("--color-background", background);
  root.style.setProperty("--color-text", text);
}

function saveTheme() {
  const mode = document.querySelector('input[name="theme-mode"]:checked').value;

  if (mode === "custom") {
    const theme = {
      mode: "custom",
      customTheme: {
        name: "Custom",
        colors: {
          primary: document.getElementById("theme-primary").value,
          secondary: document.getElementById("theme-secondary").value,
          background: document.getElementById("theme-background").value,
          text: document.getElementById("theme-text").value,
        },
      },
    };

    localStorage.setItem("themeConfig", JSON.stringify(theme));
  } else {
    localStorage.setItem("themeMode", mode);
  }

  alert("Theme settings saved!");
}

// Global event setup
function setupEventListeners() {
  document.getElementById("toggle-listening").addEventListener("click", async () => {
    try {
      if (!state.isListening) {
        await invoke("start_listening");
        state.isListening = true;
        document.getElementById("toggle-listening").textContent =
          "Stop Listening";
        document.getElementById("toggle-listening").classList.add("active");
      } else {
        await invoke("stop_listening");
        state.isListening = false;
        document.getElementById("toggle-listening").textContent =
          "Start Listening";
        document.getElementById("toggle-listening").classList.remove("active");
      }
    } catch (error) {
      console.error("Error toggling listener:", error);
    }
  });
}

// Update dashboard periodically
setInterval(updateDashboard, 2000);
