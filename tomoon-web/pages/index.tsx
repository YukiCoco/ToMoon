import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'

const Home: NextPage = () => {
  return (
    <div style={{ background:'#FAFFE3' }}>
      <div className='container grid grid-cols-12'>
        <div className='col-span-12 lg:col-start-2 lg:col-span-10'>
          <div className='flex justify-center items-center h-screen flex-col gap-4'>
            <h1 className="text-6xl">
              Tomoon
            </h1>
            <div className='w-full flex px-2'>
              <input className='tomoon-input grow lg:text-2xl text-xl pl-5' type="text" placeholder='Send your link' />
              <button className='tomoon-button text-4xl flex justify-center items-center'>
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

export default Home
