import { invoke } from "@tauri-apps/api/core";
import { TrayIcon } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import { Menu } from "@tauri-apps/api/menu";
import { exit } from '@tauri-apps/plugin-process';

Menu.new({
  items: [
    {
      id: 'start',
      text: 'Start',
      action: async () => {
        await start();
      }
    },
    {
      id: 'stop',
      text: 'Stop',
      action: async () => {
        await stop();
      }
    },
    {
      item: 'Separator',
    },
    {
      id: 'quit',
      text: 'Quit',
      action: async () => {
        await exit(0);
      }
    },
  ],
}).then((menu) => {
  defaultWindowIcon().then((icon) => {
    TrayIcon.new({
      icon: icon || undefined,
      menu,
    }).then(() => {
    });
  });
});

const btnStart = document.getElementById('start-btn');
const btnStop = document.getElementById('stop-btn');
const txtHost = document.getElementById('host');
const txtBind = document.getElementById('bind');
const txtPort = document.getElementById('port');

const stored = localStorage.getItem('config');
if (stored) {
  const config = JSON.parse(stored);
  txtHost.value = config.host;
  txtBind.value = config.bind;
  txtPort.value = config.port;
}

async function start() {
  btnStart.setAttribute('disabled', 'disabled');
  const host = txtHost.value.trim() || 'game.endless-online.com:8078';
  const bind = txtBind.value || '127.0.0.1';
  const port = Number.parseInt(txtPort.value || '8077', 10);

  localStorage.setItem('config', JSON.stringify({ host, bind, port }));

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
}

async function stop() {
  btnStop.setAttribute('disabled', 'disabled');
  await invoke('stop_proxy');
  btnStart.removeAttribute('disabled');
}

btnStart.addEventListener('click', start);
btnStop.addEventListener('click', stop);
