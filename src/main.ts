import { invoke } from "@tauri-apps/api/core";
import { TrayIcon } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import { Menu } from "@tauri-apps/api/menu";

const menu = await Menu.new({
  items: [
    {
      id: 'start',
      text: 'Start',
    },
    {
      id: 'stop',
      text: 'Stop',
    },
    {
      item: 'Separator',
    },
    {
      id: 'quit',
      text: 'Quit',
    },
  ],
});

const tray = await TrayIcon.new({
  icon: await defaultWindowIcon() || undefined,
  menu,
});

type Config = {
  host: string;
  bind: string;
  port: string;
};

window.addEventListener("DOMContentLoaded", () => {
  const btnStart = document.getElementById('start-btn')! as HTMLButtonElement;
  const btnStop = document.getElementById('stop-btn')! as HTMLButtonElement;
  const txtHost = document.getElementById('host')! as HTMLInputElement;
  const txtBind = document.getElementById('bind')! as HTMLInputElement;
  const txtPort = document.getElementById('port')! as HTMLInputElement;

  const stored = localStorage.getItem('config');
  if (stored) {
    const config = JSON.parse(stored) as Config;
    txtHost.value = config.host;
    txtBind.value = config.bind;
    txtPort.value = config.port;
  }

  btnStart.addEventListener('click', async () => {
    btnStart.setAttribute('disabled', 'disabled');
    const host = txtHost.value.trim() || 'game.endless-online.com:8078';
    const bind = txtBind.value || '127.0.0.1';
    const port = Number.parseInt(txtPort.value || '8077', 10);

    localStorage.setItem('config', JSON.stringify({host, bind, port}));

    try {
      await invoke('start_proxy', {
        bindHost: bind,
        bindPort: port,
        targetHost: host,
      });
      btnStop.removeAttribute('disabled');
    } catch (e) {
      alert(`Error: ${e}`);
      btnStart.removeAttribute('disabled');
    }
  });

  btnStop.addEventListener('click', async () => {
    btnStop.setAttribute('disabled', 'disabled');
    await invoke('stop_proxy');
    btnStart.removeAttribute('disabled');
  });
});
