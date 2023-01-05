import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'
import Swal from 'sweetalert2'

const Home: NextPage = () => {
  return (
    <div style={{ background:'#FAFFE3' }}>
      <div className='container grid grid-cols-12'>
        <div className='col-span-12 lg:col-start-2 lg:col-span-10'>
          <div className='flex justify-center items-center h-screen flex-col gap-4'>
            <h1 className="text-6xl tomoon-title">
              Tomoon
            </h1>
            <div className='w-full flex px-2'>
              <input className='tomoon-input grow lg:text-2xl text-xl pl-5' type="text" placeholder='Clash 订阅链接' />
              <button className='tomoon-button text-4xl flex justify-center items-center' onClick={on_download_btn_click}>
                <svg width="48" height="48" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M0.0228571 48L48 24L0.0228571 0L0 18.6667L34.2857 24L0 29.3333L0.0228571 48Z" fill="white" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

const on_download_btn_click = () => {
  Swal.fire({
    title: "下载中",
    text: "正在下载订阅配置，请稍等......",
    icon: "info"
  });
  Swal.showLoading(null);

  Swal.fire({
    icon: 'success',
    iconColor: '#5E5F55',
    title: '完成',
    text: '订阅下载成功！',
    confirmButtonColor: '#5A6242',
    background: '#DEE7BF'
  })
}

export default Home
