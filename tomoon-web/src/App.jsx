import { useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import Swal from 'sweetalert2'
import axios from 'axios'

function App() {
  const [url, setUrl] = useState("");
  const [isSubscribed, setIsSubscribed] = useState(true);

  const handleUrlChange = (event) => {
    setUrl(event.target.value);
  };

  const handleToggleChange = () => {
    setIsSubscribed(!isSubscribed);
  };

  return (
    <div style={{ background: '#FAFFE3', minHeight: '100vh' }} className='flex justify-center items-center'>
      <div className='container grid grid-cols-12'>
        <div className='col-span-12 lg:col-start-2 lg:col-span-10'>
          <div className='flex justify-center items-center min-h-screen flex-col gap-4'>
            <h1 className="text-6xl tomoon-title">
              Tomoon
            </h1>
            <div className="flex items-center gap-2">
              <label className="switch">
                <input type="checkbox" checked={isSubscribed} onChange={handleToggleChange} />
                <span className="slider round"></span>
              </label>
              <span className="text-xl tomoon-title">订阅转换</span>
            </div>
            <div className='w-full flex px-2'>
              <input id="input-url" className='tomoon-input grow lg:text-2xl text-xl pl-5' type="text" placeholder='Clash 订阅链接' value={url} onChange={handleUrlChange} />
              <button className='tomoon-button text-4xl flex justify-center items-center' onClick={() => { on_download_btn_click(url, isSubscribed) }}>
                <svg className="tomoon-svg" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M0.0228571 48L48 24L0.0228571 0L0 18.6667L34.2857 24L0 29.3333L0.0228571 48Z" fill="white" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

const on_download_btn_click = (url, isSubscribed) => {
  let baseHost = '/';
  if (import.meta.env.DEV) {
    baseHost = 'http://127.0.0.1:55556/';
  }
  Swal.fire({
    iconColor: '#5E5F55',
    confirmButtonColor: '#5A6242',
    background: '#DEE7BF',
    title: "下载中",
    text: "正在下载订阅配置，请稍等......",
    icon: "info"
  });
  Swal.showLoading(null);
  axios.post(baseHost + "download_sub", {
    link: url.trim(),
    subconv: isSubscribed
  }, {
    headers: { 'content-type': 'application/x-www-form-urlencoded' },
  }).then((response) => {
    if (response.status === 200) {
      Swal.fire({
        icon: 'success',
        iconColor: '#5E5F55',
        title: '完成',
        text: '已添加订阅',
        confirmButtonColor: '#5A6242',
        background: '#DEE7BF'
      });
    }
  }).catch(error => {
    if (error.response) {
      Swal.fire({
        icon: 'error',
        iconColor: '#5E5F55',
        title: '失败',
        text: error.response.data?.error?.message,
        confirmButtonColor: '#5A6242',
        background: '#DEE7BF'
      });
    }
  });

}

export default App
