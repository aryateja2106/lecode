export default function Pricing() {
  return (
    <section className="bg-[#09090B] py-24 px-6">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold text-white mb-4">
            Simple, transparent pricing
          </h2>
          <p className="text-zinc-400 text-lg">Choose the plan that fits your needs</p>
        </div>

        <div className="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto">
          {/* Free Tier */}
          <div className="border border-zinc-800 rounded-lg p-8 bg-zinc-950">
            <h3 className="text-2xl font-bold text-white mb-2">Free</h3>
            <div className="mb-6">
              <span className="text-4xl font-bold text-white">$0</span>
            </div>
            <ul className="space-y-4 mb-8">
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-zinc-400 mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>10 sessions/month</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-zinc-400 mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>$20 cloud sandbox credit</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-zinc-400 mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>Web and mobile access</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-zinc-400 mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>Community support</span>
              </li>
            </ul>
            <button className="w-full py-3 px-6 bg-zinc-800 hover:bg-zinc-700 text-white rounded-lg font-medium transition-colors">
              Start Free
            </button>
          </div>

          {/* Pro Tier */}
          <div className="border border-zinc-700 rounded-lg p-8 bg-zinc-900 relative shadow-xl ring-2 ring-zinc-700">
            <div className="absolute -top-4 left-1/2 -translate-x-1/2">
              <span className="bg-white text-zinc-950 px-4 py-1 rounded-full text-sm font-semibold">
                Most Popular
              </span>
            </div>
            <h3 className="text-2xl font-bold text-white mb-2">Pro</h3>
            <div className="mb-6">
              <span className="text-4xl font-bold text-white">$20</span>
              <span className="text-zinc-400">/month</span>
            </div>
            <ul className="space-y-4 mb-8">
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-white mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>Unlimited sessions</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-white mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>$100 cloud sandbox credit</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-white mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>Priority support</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-white mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>Locked-in price</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-white mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>All future features</span>
              </li>
            </ul>
            <button className="w-full py-3 px-6 bg-white hover:bg-zinc-100 text-zinc-950 rounded-lg font-medium transition-colors">
              Get Started
            </button>
          </div>

          {/* Enterprise Tier */}
          <div className="border border-zinc-800 rounded-lg p-8 bg-zinc-950">
            <h3 className="text-2xl font-bold text-white mb-2">Enterprise</h3>
            <div className="mb-6">
              <span className="text-4xl font-bold text-white">Custom</span>
            </div>
            <ul className="space-y-4 mb-8">
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-zinc-400 mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>Unlimited</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-zinc-400 mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>Team collaboration</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-zinc-400 mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>Priority support</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-zinc-400 mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>Custom integrations</span>
              </li>
              <li className="flex items-start text-zinc-300">
                <svg
                  className="w-5 h-5 text-zinc-400 mr-3 mt-0.5 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
                <span>99.9% uptime SLA</span>
              </li>
            </ul>
            <button className="w-full py-3 px-6 bg-zinc-800 hover:bg-zinc-700 text-white rounded-lg font-medium transition-colors">
              Schedule a Call
            </button>
          </div>
        </div>
      </div>
    </section>
  );
}
