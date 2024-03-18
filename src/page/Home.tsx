import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

interface License {
  value: string;
  expire: string;
  username: string;
  hwid: string;
}

interface Config {
  server_ip: string,
  stream_uri: string,
  fps: number;
  encoding: string;
  dimensions: string;
}

export default function HomePage() {
  const [license, setLicense] = useState<License>();
  const [config, setConfig] = useState<Config>({
    server_ip: '',
    stream_uri: '',
    fps: 30,
    encoding: '',
    dimensions: '',
  });

  useEffect(() => {
    async function fetchData() {
      try {
        const licenseData: License = await invoke('get_license');
        setLicense(licenseData);
        const configData: Config = await invoke('get_config');
        setConfig(configData);
      } catch (error) {
        console.error("Error fetching data:", error);
      }
    }
    fetchData();
  }, []);

  async function revoke() {
    try {
      await invoke('revoke');
    } catch (error) {
      console.error("Error revoking license:", error);
    }
  }

  async function updateConfig() {
    try {
      await invoke('set_config', { config });
    } catch (error) {
      console.error("Error setting config:", error);
    }
  }

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    setConfig(prevConfig => ({ ...prevConfig, [name]: value }));
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    await updateConfig();
  };

  return (
    <div>
      <h1>Welcome {license?.username}!</h1>
      <div>
      <p>Your license is <code>{license?.value}</code></p>
      <p>hwid is <code>{license?.hwid}</code></p>
      <p>Expiration date <code>{license?.expire}</code></p>
      <button onClick={revoke}>Revoke license</button>

      </div>
      <div>
        <h1>Settings</h1>
        <label>
        Server IP:
        <input type="text" name="server_ip" value={config.server_ip} onChange={handleChange} />
        </label>
        <label>
        Stream URI:
        <input type="text" name="stream_uri" value={config.stream_uri} onChange={handleChange} />
        </label>
      <form onSubmit={handleSubmit}>
        <label>
          FPS:
          <input type="number" name="fps" value={config.fps.toString()} onChange={handleChange} />
        </label>
        <label>
          Encoding:
          <input type="text" name="encoding" value={config.encoding} onChange={handleChange} />
        </label>
        <label>
          Dimensions:
          <input type="text" name="dimensions" value={config.dimensions} onChange={handleChange} />
        </label>
        <button type="submit">Update Config</button>
      </form>
      </div>
      
    </div>
  );
}
