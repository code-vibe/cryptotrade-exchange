import React, { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useMutation } from '@tanstack/react-query';
import {
  UserIcon,
  ShieldCheckIcon,
  BellIcon,
  CogIcon,
} from '@heroicons/react/24/outline';
import QRCode from 'qrcode.react';
import toast from 'react-hot-toast';
import { apiClient } from '../../services/api';
import { useAuthStore, useUIStore } from '../../stores';

const Settings: React.FC = () => {
  const [activeTab, setActiveTab] = useState('profile');
  const [show2FASetup, setShow2FASetup] = useState(false);
  const [qrCodeUrl, setQrCodeUrl] = useState('');

  const { user, updateUser } = useAuthStore();
  const { theme, toggleTheme } = useUIStore();

  const tabs = [
    { id: 'profile', name: 'Profile', icon: UserIcon },
    { id: 'security', name: 'Security', icon: ShieldCheckIcon },
    { id: 'notifications', name: 'Notifications', icon: BellIcon },
    { id: 'preferences', name: 'Preferences', icon: CogIcon },
  ];

  // 2FA Setup
  const enable2FAMutation = useMutation({
    mutationFn: () => apiClient.enable2FA(),
    onSuccess: (response) => {
      setQrCodeUrl(response.qrCodeUrl);
      setShow2FASetup(true);
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.message || 'Failed to enable 2FA');
    },
  });

  const confirm2FASchema = z.object({
    totpCode: z.string().length(6, '2FA code must be 6 digits'),
  });

  const {
    register: register2FA,
    handleSubmit: handleSubmit2FA,
    formState: { errors: errors2FA },
  } = useForm({
    resolver: zodResolver(confirm2FASchema),
  });

  const confirm2FAMutation = useMutation({
    mutationFn: (data: { totpCode: string }) => apiClient.confirm2FA(data),
    onSuccess: () => {
      setShow2FASetup(false);
      updateUser({ twoFaEnabled: true });
      toast.success('2FA enabled successfully!');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.message || 'Invalid 2FA code');
    },
  });

  const disable2FAMutation = useMutation({
    mutationFn: (data: { totpCode: string }) => apiClient.disable2FA(data),
    onSuccess: () => {
      updateUser({ twoFaEnabled: false });
      toast.success('2FA disabled successfully!');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.message || 'Failed to disable 2FA');
    },
  });

  const renderProfileTab = () => (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium text-gray-900 dark:text-white">Profile Information</h3>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Update your account information and email address.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div>
          <label className="form-label">Email</label>
          <input
            type="email"
            value={user?.email || ''}
            className="form-input"
            disabled
          />
        </div>

        <div>
          <label className="form-label">Username</label>
          <input
            type="text"
            value={user?.username || ''}
            className="form-input"
            disabled
          />
        </div>

        <div>
          <label className="form-label">First Name</label>
          <input
            type="text"
            value={user?.firstName || ''}
            className="form-input"
            placeholder="Enter first name"
          />
        </div>

        <div>
          <label className="form-label">Last Name</label>
          <input
            type="text"
            value={user?.lastName || ''}
            className="form-input"
            placeholder="Enter last name"
          />
        </div>
      </div>

      <div className="flex justify-end">
        <button className="btn-primary">
          Save Changes
        </button>
      </div>
    </div>
  );

  const renderSecurityTab = () => (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium text-gray-900 dark:text-white">Security Settings</h3>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Manage your account security and authentication methods.
        </p>
      </div>

      {/* 2FA Section */}
      <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-6">
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-base font-medium text-gray-900 dark:text-white">
              Two-Factor Authentication
            </h4>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Add an extra layer of security to your account
            </p>
          </div>
          <div className="flex items-center space-x-3">
            <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
              user?.twoFaEnabled 
                ? 'text-green-800 bg-green-100 dark:text-green-400 dark:bg-green-900'
                : 'text-red-800 bg-red-100 dark:text-red-400 dark:bg-red-900'
            }`}>
              {user?.twoFaEnabled ? 'Enabled' : 'Disabled'}
            </span>
            {!user?.twoFaEnabled ? (
              <button
                onClick={() => enable2FAMutation.mutate()}
                disabled={enable2FAMutation.isPending}
                className="btn-primary"
              >
                Enable 2FA
              </button>
            ) : (
              <button
                onClick={() => {
                  const code = prompt('Enter your 2FA code to disable:');
                  if (code) disable2FAMutation.mutate({ totpCode: code });
                }}
                className="btn-danger"
              >
                Disable 2FA
              </button>
            )}
          </div>
        </div>

        {/* 2FA Setup Modal */}
        {show2FASetup && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                Set up Two-Factor Authentication
              </h3>

              <div className="text-center mb-4">
                <QRCode value={qrCodeUrl} size={200} />
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-2">
                  Scan this QR code with your authenticator app
                </p>
              </div>

              <form onSubmit={handleSubmit2FA((data) => confirm2FAMutation.mutate(data))}>
                <div className="mb-4">
                  <label className="form-label">Enter 6-digit code</label>
                  <input
                    type="text"
                    maxLength={6}
                    className="form-input text-center"
                    placeholder="000000"
                    {...register2FA('totpCode')}
                  />
                  {errors2FA.totpCode && (
                    <p className="mt-1 text-sm text-red-600">{errors2FA.totpCode.message}</p>
                  )}
                </div>

                <div className="flex space-x-3">
                  <button
                    type="button"
                    onClick={() => setShow2FASetup(false)}
                    className="flex-1 btn-secondary"
                  >
                    Cancel
                  </button>
                  <button
                    type="submit"
                    disabled={confirm2FAMutation.isPending}
                    className="flex-1 btn-primary"
                  >
                    Verify & Enable
                  </button>
                </div>
              </form>
            </div>
          </div>
        )}
      </div>

      {/* Change Password */}
      <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-6">
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-base font-medium text-gray-900 dark:text-white">
              Change Password
            </h4>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Update your account password
            </p>
          </div>
          <button className="btn-secondary">
            Change Password
          </button>
        </div>
      </div>
    </div>
  );

  const renderPreferencesTab = () => (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium text-gray-900 dark:text-white">Preferences</h3>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Customize your trading experience.
        </p>
      </div>

      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-base font-medium text-gray-900 dark:text-white">Dark Mode</h4>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Switch between light and dark themes
            </p>
          </div>
          <button
            onClick={toggleTheme}
            className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 ${
              theme === 'dark' ? 'bg-primary-600' : 'bg-gray-200'
            }`}
          >
            <span
              className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                theme === 'dark' ? 'translate-x-5' : 'translate-x-0'
              }`}
            />
          </button>
        </div>
      </div>
    </div>
  );

  return (
    <div className="p-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Settings</h1>
        <p className="text-gray-600 dark:text-gray-400">
          Manage your account settings and preferences
        </p>
      </div>

      <div className="flex">
        {/* Sidebar */}
        <div className="w-64 mr-8">
          <nav className="space-y-1">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`w-full flex items-center px-3 py-2 text-sm font-medium rounded-md ${
                  activeTab === tab.id
                    ? 'bg-primary-100 text-primary-900 dark:bg-primary-900 dark:text-primary-100'
                    : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900 dark:text-gray-300 dark:hover:bg-gray-700 dark:hover:text-white'
                }`}
              >
                <tab.icon className="mr-3 h-5 w-5" />
                {tab.name}
              </button>
            ))}
          </nav>
        </div>

        {/* Content */}
        <div className="flex-1 bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          {activeTab === 'profile' && renderProfileTab()}
          {activeTab === 'security' && renderSecurityTab()}
          {activeTab === 'preferences' && renderPreferencesTab()}
          {activeTab === 'notifications' && (
            <div className="text-center py-12">
              <BellIcon className="mx-auto h-12 w-12 text-gray-400" />
              <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
                Notification Settings
              </h3>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                Configure your notification preferences (Coming Soon)
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default Settings;
